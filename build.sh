# set the name of the Mac App
APP_NAME="Confetti Desktop"
# set the name of your rust crate
RUST_CRATE_NAME="confetti_desktop"
# create the folder structure
mkdir -p "${APP_NAME}.app/Contents/MacOS"
mkdir -p "${APP_NAME}.app/Contents/Resources"
# copy Info.plist
cp Info.plist "${APP_NAME}.app/Contents/Info.plist"
# copy the icon (assuming you already have it in Apple ICNS format)
cp AppIcon.icns "${APP_NAME}.app/Contents/Resources/AppIcon.icns"
# copy your Bevy game assets
cp -a assets "${APP_NAME}.app/Contents/MacOS/"
# compile the executables for each architecture
cargo build --release --target x86_64-apple-darwin # build for Intel
cargo build --release --target aarch64-apple-darwin # build for Apple Silicon
# combine the executables into a single file and put it in the bundle
lipo "/Users/joshvoigts/builds/x86_64-apple-darwin/release/${RUST_CRATE_NAME}" \
     "/Users/joshvoigts/builds/aarch64-apple-darwin/release/${RUST_CRATE_NAME}" \
     -create -output "${APP_NAME}.app/Contents/MacOS/${RUST_CRATE_NAME}"

# compile for windows
cargo build --target=x86_64-pc-windows-gnu --release
