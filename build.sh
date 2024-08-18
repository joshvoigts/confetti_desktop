# set the name of the Mac App
APP_NAME="Confetti Desktop"
# set the name of your rust crate
RUST_CRATE_NAME="confetti_desktop"
# get release directory
TARGET_DIR="$(cargo metadata | jq -r ".target_directory")"
RELEASE_DIR="$TARGET_DIR/${RUST_CRATE_NAME}"
APP_DIR="$RELEASE_DIR/${APP_NAME}.app"
# create the folder structure
mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Resources"
# copy Info.plist
cp Info.plist "$APP_DIR/Contents/Info.plist"
# copy the icon (assuming you already have it in Apple ICNS format)
cp AppIcon.icns "$APP_DIR/Contents/Resources/AppIcon.icns"
# copy your Bevy game assets
# cp -a assets "$APP_DIR/Contents/MacOS/"
# compile the executables for each architecture
cargo build --release --target x86_64-apple-darwin # build for Intel
cargo build --release --target aarch64-apple-darwin # build for Apple Silicon
# combine the executables into a single file and put it in the bundle
lipo "$TARGET_DIR/x86_64-apple-darwin/release/${RUST_CRATE_NAME}" \
     "$TARGET_DIR/aarch64-apple-darwin/release/${RUST_CRATE_NAME}" \
     -create -output "$APP_DIR/Contents/MacOS/${RUST_CRATE_NAME}"

# adhoc sign the app
sudo codesign --force --sign - --deep "$APP_DIR"

zip -r "$RELEASE_DIR/${APP_NAME}-macos.zip" "$RELEASE_DIR/${APP_NAME}.app"

# compile for windows
cargo build --target=x86_64-pc-windows-gnu --release
cp "$TARGET_DIR/x86_64-pc-windows-gnu/release/confetti_desktop.exe" "$RELEASE_DIR/${APP_NAME}.exe"
zip "$RELEASE_DIR/${APP_NAME}-windows.zip" "$RELEASE_DIR/${APP_NAME}.exe"
