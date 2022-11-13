cd ../
mkdir NDK
cd NDK

%NDK_HOME%/build/tools/make_standalone_toolchain.py --api 26 --arch arm64 --install-dir arm64
%NDK_HOME%/build/tools/make_standalone_toolchain.py --api 26 --arch arm --install-dir arm
%NDK_HOME%/build/tools/make_standalone_toolchain.py --api 26 --arch x86 --install-dir x86
%NDK_HOME%/build/tools/make_standalone_toolchain.py --api 26 --arch x86_64 --install-dir x86_64

rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

cd arm\lib64\clang\
for /D %%f in (*) do (
    set clang-version=%%f
    goto :term
) 
:term

cd ../../../../initial_setup
set libgcc=%cd%
cd ../NDK

copy /y %libgcc%\libgcc.a %cd%\arm\lib64\clang\%clang-version%\lib\linux\aarch64\libgcc.a
copy /y %libgcc%\libgcc.a %cd%\arm\lib64\clang\%clang-version%\lib\linux\arm\libgcc.a
copy /y %libgcc%\libgcc.a %cd%\arm\lib64\clang\%clang-version%\lib\linux\i386\libgcc.a
copy /y %libgcc%\libgcc.a %cd%\arm\lib64\clang\%clang-version%\lib\linux\x86_64\libgcc.a

copy /y %libgcc%\libgcc.a %cd%\arm64\lib64\clang\%clang-version%\lib\linux\aarch64\libgcc.a
copy /y %libgcc%\libgcc.a %cd%\arm64\lib64\clang\%clang-version%\lib\linux\arm\libgcc.a
copy /y %libgcc%\libgcc.a %cd%\arm64\lib64\clang\%clang-version%\lib\linux\i386\libgcc.a
copy /y %libgcc%\libgcc.a %cd%\arm64\lib64\clang\%clang-version%\lib\linux\x86_64\libgcc.a

copy /y %libgcc%\libgcc.a %cd%\x86\lib64\clang\%clang-version%\lib\linux\aarch64\libgcc.a
copy /y %libgcc%\libgcc.a %cd%\x86\lib64\clang\%clang-version%\lib\linux\arm\libgcc.a
copy /y %libgcc%\libgcc.a %cd%\x86\lib64\clang\%clang-version%\lib\linux\i386\libgcc.a
copy /y %libgcc%\libgcc.a %cd%\x86\lib64\clang\%clang-version%\lib\linux\x86_64\libgcc.a

copy /y %libgcc%\libgcc.a %cd%\x86_64\lib64\clang\%clang-version%\lib\linux\aarch64\libgcc.a
copy /y %libgcc%\libgcc.a %cd%\x86_64\lib64\clang\%clang-version%\lib\linux\arm\libgcc.a
copy /y %libgcc%\libgcc.a %cd%\x86_64\lib64\clang\%clang-version%\lib\linux\i386\libgcc.a
copy /y %libgcc%\libgcc.a %cd%\x86_64\lib64\clang\%clang-version%\lib\linux\x86_64\libgcc.a
