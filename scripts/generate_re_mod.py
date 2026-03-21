import os
import glob

def main():
    # Определяем пути (скрипт лежит в /scripts/)
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_root = os.path.abspath(os.path.join(script_dir, ".."))
    re_dir = os.path.join(project_root, "libskyrim", "src", "re")
    mod_file_path = os.path.join(re_dir, "mod.rs")

    # Проверяем, существует ли папка re
    if not os.path.exists(re_dir):
        print(f"Ошибка: Папка {re_dir} не найдена!")
        return

    # Ищем все .rs файлы
    rs_files = glob.glob(os.path.join(re_dir, "*.rs"))

    modules = []
    for filepath in rs_files:
        filename = os.path.basename(filepath)
        if filename != "mod.rs":
            mod_name = filename[:-3] # Отрезаем расширение .rs
            modules.append(mod_name)

    # Сортируем по алфавиту для красоты и стабильности
    modules.sort()

    # Записываем в mod.rs
    with open(mod_file_path, "w", encoding="utf-8") as f:
        f.write("//! =============================================================================\n")
        f.write("//! Этот файл сгенерирован АВТОМАТИЧЕСКИ скриптом `scripts/generate_re_mod.py`.\n")
        f.write("//! Не редактируйте его вручную, иначе изменения будут потеряны при перегенерации.\n")
        f.write("//! =============================================================================\n\n")

        # Блок pub mod
        for mod in modules:
            f.write(f"pub mod {mod};\n")

        f.write("\n")

        # Блок pub use
        for mod in modules:
            f.write(f"pub use {mod}::*;\n")

    print(f"Успешно! Сгенерирован {mod_file_path} (модулей: {len(modules)})")

if __name__ == "__main__":
    main()