import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'bridge_generated.dart';

class RustLib {
  const RustLib._();
  static FlutterRust instance = FlutterRustImpl(loadDylib('rust_native.dll'));
}
