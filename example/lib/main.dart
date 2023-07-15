import 'dart:developer';

import 'package:flutter/material.dart';
import 'dart:async';
import 'package:flutter_rust/flutter_rust.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  String platform = "";

  List<BluetoothDevice> bluetoothDevice = [];

  @override
  void initState() {
    try {
      FlutterRustPlugin().initLogger();
    } catch (e) {
      log(e.toString());
    }
    getPlatform();
    super.initState();
  }

  Future<void> getPlatform() async {
    final info = await FlutterRustPlugin().getPlatformVersion();
    setState(() {
      platform = info.edition;
    });
  }

  Future<void> streamBluetooth() async {
    FlutterRustPlugin()
        .bluetoothStartScan(timeoutSec: const Duration(seconds: 15))
        .listen((event) {
      setState(() {
        bluetoothDevice.add(event);
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Plugin example app'),
        ),
        body: ListView.builder(
          itemCount: bluetoothDevice.length,
          itemBuilder: (context, index) {
            return Text(bluetoothDevice[index].name ?? '-');
          },
        ),
        floatingActionButton: FloatingActionButton(
          onPressed: () async {
            await streamBluetooth();
          },
          child: const Text('Scan'),
        ),
      ),
    );
  }
}
