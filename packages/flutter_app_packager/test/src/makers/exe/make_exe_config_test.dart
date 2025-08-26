import 'dart:io';

import 'package:flutter_app_packager/src/makers/exe/make_exe_config.dart';
import 'package:pub_semver/pub_semver.dart';
import 'package:pubspec_parse/pubspec_parse.dart';
import 'package:test/test.dart';

void main() {
  group('MakeExeConfig', () {
    test('fromJson should parse inno_setup_path correctly', () {
      final json = {
        'app_id': 'test-app-id',
        'inno_setup_path': 'D:\\CustomPath\\Inno Setup 6',
        'display_name': 'Test App',
        'publisher_name': 'Test Publisher',
        'locales': ['en', 'zh'],
      };

      final config = MakeExeConfig.fromJson(json);

      expect(config.appId, equals('test-app-id'));
      expect(config.innoSetupPath, equals('D:\\CustomPath\\Inno Setup 6'));
      expect(config.displayName, equals('Test App'));
      expect(config.publisherName, equals('Test Publisher'));
      expect(config.locales, equals(['en', 'zh']));
    });

    test('fromJson should handle missing inno_setup_path', () {
      final json = {
        'app_id': 'test-app-id',
        'display_name': 'Test App',
        'locales': ['en'],
      };

      final config = MakeExeConfig.fromJson(json);

      expect(config.appId, equals('test-app-id'));
      expect(config.innoSetupPath, isNull);
      expect(config.displayName, equals('Test App'));
    });

    test('toJson should include inno_setup_path when present', () {
      final config = MakeExeConfig(
        appId: 'test-app-id',
        innoSetupPath: 'D:\\CustomPath\\Inno Setup 6',
        displayName: 'Test App',
        publisherName: 'Test Publisher',
        locales: ['en'],
      );

      // Set required fields for the base config
      config.buildOutputDirectory = Directory('build');
      config.buildOutputFiles = [];
      config.platform = 'windows';
      config.packageFormat = 'exe';
      config.outputDirectory = Directory('dist/');
      config.pubspec = Pubspec(
        'test_app',
        version: Version.parse('1.0.0'),
      );

      final json = config.toJson();

      expect(json['app_id'], equals('test-app-id'));
      expect(json['inno_setup_path'], equals('D:\\CustomPath\\Inno Setup 6'));
      expect(json['display_name'], equals('Test App'));
      expect(json['publisher_name'], equals('Test Publisher'));
    });

    test('toJson should exclude null inno_setup_path', () {
      final config = MakeExeConfig(
        appId: 'test-app-id',
        displayName: 'Test App',
        locales: ['en'],
      );

      // Set required fields for the base config
      config.buildOutputDirectory = Directory('build');
      config.buildOutputFiles = [];
      config.platform = 'windows';
      config.packageFormat = 'exe';
      config.outputDirectory = Directory('dist/');
      config.pubspec = Pubspec(
        'test_app',
        version: Version.parse('1.0.0'),
      );

      final json = config.toJson();

      expect(json['app_id'], equals('test-app-id'));
      expect(json.containsKey('inno_setup_path'), isFalse);
      expect(json['display_name'], equals('Test App'));
    });
  });
}