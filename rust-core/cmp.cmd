cargo build --target aarch64-linux-android --release --color always 
cargo build --target armv7-linux-androideabi --release --color always 
cargo build --target i686-linux-android --release --color always 
cargo build --target x86_64-linux-android --release --color always 

copy /y D:\Development\project\rust-core\target\aarch64-linux-android\release\librust_core.so D:\Development\project\android\Project\app\src\main\JNILibs\arm64-v8a\librust_core.so 
copy /y D:\Development\project\rust-core\target\armv7-linux-androideabi\release\librust_core.so D:\Development\project\android\Project\app\src\main\JNILibs\armeabi-v7a\librust_core.so 
copy /y D:\Development\project\rust-core\target\x86_64-linux-android\release\librust_core.so D:\Development\project\android\Project\app\src\main\JNILibs\x86_64\librust_core.so 
copy /y D:\Development\project\rust-core\target\i686-linux-android\release\librust_core.so D:\Development\project\android\Project\app\src\main\JNILibs\x86\librust_core.so 
