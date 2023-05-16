with import <nixpkgs> {};
mkShell {
  name = "gst-openai";
  buildInputs = [
    cargo
    cargo-c
    cmake
    cacert
    pkg-config
    git
    glib
    gst_all_1.gstreamer
    gst_all_1.gst-plugins-base
  ] ++ (if stdenv.isDarwin then [
    darwin.apple_sdk.frameworks.Security
  ] else []);
}
