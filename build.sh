APP_NAME="Confetti Desktop"
RUST_CRATE_NAME="confetti_desktop"
TARGET_DIR="$(cargo metadata | jq -r ".target_directory")"
RELEASE_DIR="$TARGET_DIR/${RUST_CRATE_NAME}"
APP_DIR="$RELEASE_DIR/${APP_NAME}.app"

# compile for macos
mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Resources"
cp Info.plist "$APP_DIR/Contents/Info.plist"
cp AppIcon.icns "$APP_DIR/Contents/Resources/AppIcon.icns"
# cp -a assets "$APP_DIR/Contents/MacOS/"
cargo build --release --target x86_64-apple-darwin # build for Intel
cargo build --release --target aarch64-apple-darwin # build for Apple Silicon
# combine the executables into a single file and put it in the bundle
lipo "$TARGET_DIR/x86_64-apple-darwin/release/${RUST_CRATE_NAME}" \
     "$TARGET_DIR/aarch64-apple-darwin/release/${RUST_CRATE_NAME}" \
     -create -output "$APP_DIR/Contents/MacOS/${RUST_CRATE_NAME}"
# adhoc sign the app
sudo codesign --force --sign - --deep "$APP_DIR"

# compile for windows
cargo build --target=x86_64-pc-windows-gnu --release
cp "$TARGET_DIR/x86_64-pc-windows-gnu/release/confetti_desktop.exe" "$RELEASE_DIR/${APP_NAME}.exe"

# zip up executables
cd "$RELEASE_DIR"
zip -r "$RELEASE_DIR/${APP_NAME}-macos.zip" "${APP_NAME}.app"
zip "$RELEASE_DIR/${APP_NAME}-windows.zip" "${APP_NAME}.exe"
