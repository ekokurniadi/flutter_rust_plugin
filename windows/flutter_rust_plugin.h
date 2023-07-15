#ifndef FLUTTER_PLUGIN_FLUTTER_RUST_PLUGIN_H_
#define FLUTTER_PLUGIN_FLUTTER_RUST_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

namespace flutter_rust {

class FlutterRustPlugin : public flutter::Plugin {
 public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows *registrar);

  FlutterRustPlugin();

  virtual ~FlutterRustPlugin();

  // Disallow copy and assign.
  FlutterRustPlugin(const FlutterRustPlugin&) = delete;
  FlutterRustPlugin& operator=(const FlutterRustPlugin&) = delete;

 private:
  // Called when a method is called on this plugin's channel from Dart.
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);
};

}  // namespace flutter_rust

#endif  // FLUTTER_PLUGIN_FLUTTER_RUST_PLUGIN_H_
