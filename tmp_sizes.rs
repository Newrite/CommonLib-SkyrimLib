use std::mem::size_of;
fn main() {
    println!("VMHandle {}", size_of::<libskyrim::re::VMHandle>());
    println!("BSFixedString {}", size_of::<libskyrim::re::BSFixedString>());
    println!("BSTSmartPointer<ObjectTypeInfo> {}", size_of::<libskyrim::re::BSTSmartPointer<libskyrim::re::ObjectTypeInfo>>());
    println!("Variable {}", size_of::<libskyrim::re::Variable>());
    println!("BSTSmallArray<MemoryPageData,3> {}", size_of::<libskyrim::re::BSTSmallArray<libskyrim::re::MemoryPageData,3>>());
    println!("BSTSmartPointer<CodeTasklet> {}", size_of::<libskyrim::re::BSTSmartPointer<libskyrim::re::CodeTasklet>>());
    println!("BSTSmartPointer<IStackCallbackFunctor> {}", size_of::<libskyrim::re::BSTSmartPointer<libskyrim::re::IStackCallbackFunctor>>());
    println!("Stack {}", size_of::<libskyrim::re::Stack>());
    println!("Object {}", size_of::<libskyrim::re::Object>());
}
