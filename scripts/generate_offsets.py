import re
import os

def convert_rtti(h_file, rs_file):
    if not os.path.exists(h_file):
        print(f"Файл {os.path.basename(h_file)} не найден, пропускаем.")
        return

    with open(h_file, 'r', encoding='utf-8') as f:
        content = f.read()

    # Ищем строки вида: constexpr REL::VariantID RTTI_Name(SE_ID, AE_ID, VR_OFFSET);
    pattern = re.compile(r'constexpr REL::VariantID\s+(\w+)\(([^,]+),\s*([^,]+),\s*([^)]+)\);')

    with open(rs_file, 'w', encoding='utf-8') as f:
        f.write('// AUTO-GENERATED FILE\n')
        f.write('#![allow(non_upper_case_globals)]\n')
        f.write('use crate::relocation::VariantID;\n\n')

        count = 0
        for match in pattern.finditer(content):
            name, se, ae, vr = match.groups()
            f.write(f'pub const {name}: VariantID = VariantID::new({se.strip()}, {ae.strip()}, {vr.strip()});\n')
            count += 1

        print(f"Сгенерировано {count} RTTI констант в {os.path.basename(rs_file)}")

def convert_vtable(h_file, rs_file):
    if not os.path.exists(h_file):
        print(f"Файл {os.path.basename(h_file)} не найден, пропускаем.")
        return

    with open(h_file, 'r', encoding='utf-8') as f:
        content = f.read()

    # Ищем массивы: constexpr std::array<REL::VariantID, N> VTABLE_Name{ REL::VariantID(1,2,3), ... };
    pattern_array = re.compile(r'constexpr std::array<REL::VariantID,\s*(\d+)>\s+(\w+)\s*\{\s*(.*?)\s*\};', re.DOTALL)
    pattern_item = re.compile(r'REL::VariantID\(([^,]+),\s*([^,]+),\s*([^)]+)\)')

    with open(rs_file, 'w', encoding='utf-8') as f:
        f.write('// AUTO-GENERATED FILE\n')
        f.write('#![allow(non_upper_case_globals)]\n')
        f.write('use crate::relocation::VariantID;\n\n')

        count = 0
        for match in pattern_array.finditer(content):
            arr_size, name, items_str = match.groups()

            items = []
            for item_match in pattern_item.finditer(items_str):
                se, ae, vr = item_match.groups()
                items.append(f'VariantID::new({se.strip()}, {ae.strip()}, {vr.strip()})')

            items_rust = ",\n    ".join(items)
            f.write(f'pub const {name}: [VariantID; {arr_size}] = [\n    {items_rust}\n];\n\n')
            count += 1

        print(f"Сгенерировано {count} VTABLE массивов в {os.path.basename(rs_file)}")

def generate_mod_rs(output_dir, modules):
    """Автоматически генерирует файл mod.rs для папки offsets"""
    mod_rs_path = os.path.join(output_dir, 'mod.rs')
    with open(mod_rs_path, 'w', encoding='utf-8') as f:
        f.write('// AUTO-GENERATED FILE\n')
        for mod in modules:
            f.write(f'pub mod {mod};\n')
    print(f"Сгенерирован файл {os.path.basename(mod_rs_path)}")

if __name__ == "__main__":
    print("Начинаем конвертацию...")

    # 1. Вычисляем абсолютные пути, опираясь на расположение самого скрипта
    script_dir = os.path.dirname(os.path.abspath(__file__)) # папка /scripts
    root_dir = os.path.dirname(script_dir) # корень репозитория

    input_dir = os.path.join(script_dir, 'offsets')
    output_dir = os.path.join(root_dir, 'libskyrim', 'src', 'offsets')

    # 2. Создаем выходную папку libskyrim/src/offsets, если её вдруг нет
    os.makedirs(output_dir, exist_ok=True)

    # 3. Формируем полные пути к файлам
    rtti_in = os.path.join(input_dir, 'Offsets_RTTI.h')
    rtti_out = os.path.join(output_dir, 'offsets_rtti.rs')

    nirtti_in = os.path.join(input_dir, 'Offsets_NiRTTI.h')
    nirtti_out = os.path.join(output_dir, 'offsets_nirtti.rs')

    vtable_in = os.path.join(input_dir, 'Offsets_VTABLE.h')
    vtable_out = os.path.join(output_dir, 'offsets_vtable.rs')

    # 4. Запускаем конвертацию
    convert_rtti(rtti_in, rtti_out)
    convert_rtti(nirtti_in, nirtti_out)
    convert_vtable(vtable_in, vtable_out)

    # 5. Генерируем mod.rs, чтобы Rust сразу увидел эти файлы как модули
    generate_mod_rs(output_dir, ['offsets_rtti', 'offsets_nirtti', 'offsets_vtable'])

    print(f"\nВсе файлы успешно сохранены в: {output_dir}")