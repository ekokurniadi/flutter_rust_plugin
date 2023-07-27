import 'package:flutter_rust/generated/bridge_generated.dart';
import 'package:flutter_rust/generated/rust.dart';

class FlutterRustPlugin {
  Future<WindowsOSInfo> getPlatformVersion() async {
    final info = await RustLib.instance.getWindowsInfo();
    return info;
  }

  Future<LocalIP> findMyLocalIp() async {
    final result = await RustLib.instance.findMyLocalIp();
    return result;
  }

  Future<void> initLogger() async {
    await RustLib.instance.init();
  }

  Future<bool> getBluetoothAdapterState() async {
    return await RustLib.instance.getBluetoothAdapterState();
  }

  Future<bool> connectToBluetoothDevice({
    required String serviceUuid,
  }) async {
    return await RustLib.instance.connectToBluetoothDevice(
      serviceUuid: serviceUuid,
    );
  }

  Future<bool> disconnectBluetoothDevice({required String serviceUuid}) async {
    return await RustLib.instance.disconnectBluetoothDevice(
      serviceUuid: serviceUuid,
    );
  }

  Stream<BluetoothDevice> bluetoothStartScan({
    required Duration timeoutSec,
  }) {
    return RustLib.instance
        .bluetoothStartScan(timeoutSec: timeoutSec)
        .asyncMap<BluetoothDevice>((event) async {
      return event;
    });
  }

  Future<void> bluetoothStopScan() async {
    return await RustLib.instance.bluetoothStopScan();
  }

  Future<CameraScanner> scannerProcessImage({
    required String imagePath,
  }) async {
    return await RustLib.instance.scannerProcessImage(
      imagePath: imagePath,
    );
  }

  Future<String> invokeMethod({
    required MethodChannel channel,
  }) async {
    return await RustLib.instance.invokeMethod(channel: channel);
  }
}
