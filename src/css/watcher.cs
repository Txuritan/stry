using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace Watcher {
    class Program {
        private static Dictionary<String, String> cfg = LoadConfig("watcher.ini");
        private static string fullPattern = @"({})";
        private static string filePattern = @"({\.})";

        static void Main(string[] args) {
            var dir = Directory.GetCurrentDirectory();

            Console.WriteLine($"[Watcher] Watching folder: {dir}");

            FileSystemWatcher watcher = new FileSystemWatcher {
                Path = dir,
                Filter = "*.*",
            };

            watcher.Changed += OnChanged;

            watcher.EnableRaisingEvents = true;

            Console.WriteLine("[Watcher] Press 'q' to quit.");
            while (Console.Read() != 'q');
        }

        private static void OnChanged(object sender, FileSystemEventArgs e) {
            Console.WriteLine($"[Watcher] File changed: {e.Name}");

            var cmd = cfg["command"];

            cmd = Regex.Replace(cmd, fullPattern, Path.GetFileName(e.FullPath));
            cmd = Regex.Replace(cmd, filePattern, Path.GetFileNameWithoutExtension(e.FullPath));

            if (Regex.Match(e.Name, cfg["filter"])) {
                try {
                    Console.WriteLine($"[Watcher] Running command: {cmd}");

                    new Process {
                        StartInfo = new ProcessStartInfo {
                            FileName = "cmd.exe",
                            Arguments = $"/C {cmd}",
                            WindowStyle = ProcessWindowStyle.Hidden,
                        },
                    }.Start();
                } catch (Exception err) {
                    Console.WriteLine(err);
                }
            }
        }

        private static Dictionary<String, String> LoadConfig(string settingfile) {
            var dic = new Dictionary<String, String>();

            if (File.Exists(settingfile)) {
                var data = File.ReadAllLines(settingfile);

                for (var i = 0; i < data.Length; i++) {
                    var setting = data[i];
                    var eq_i = setting.IndexOf("=");

                    if (eq_i >= 0) {
                        var key = setting.Substring(0, eq_i).Trim();
                        var value = setting.Substring(eq_i + 1).Trim();

                        if (!dic.ContainsKey(key)) {
                            dic.Add(key, value);
                        }
                    }
                }
            }

            return dic;
        }
    }
}