// xxUSBSentinel
// Version: 1.0
// TODO: Add RAM wipe.
// TODO: Add hybernation, spawon and pagefile deletion.
// TODO: Add configuration saving.
// TODO: Pack libraries with release executable.

using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Text;
using System.Windows.Forms;
using System.Collections.ObjectModel;
using LibUsbDotNet;
using LibUsbDotNet.Info;
using LibUsbDotNet.Main;
using LibUsbDotNet.DeviceNotify;
using System.IO;
using System.Diagnostics;

namespace xxUSBSentinel {
    public partial class Form1 : Form {

        public static UsbDevice MyUsbDevice;
        public static IDeviceNotifier UsbDeviceNotifier = DeviceNotifier.OpenDeviceNotifier();
        public string CONNECTED = "Connected";
        public string DISCONNECTED = "Disconnected";
        public string Data, VID, PID, Action, KeyDevice = "";
        public bool Armed, TestMode, Waiting = false;
        public const bool Debug = false;

        #region "Helpers"
        public string Between(string STR, string FirstString, string LastString) {
            string FinalString;
            int Pos1 = STR.IndexOf(FirstString) + FirstString.Length;
            int Pos2 = STR.IndexOf(LastString);
            FinalString = STR.Substring(Pos1, Pos2 - Pos1);
            return FinalString;
        }
        public string After(string STR, string AFTER) {
            return STR.Substring(0, STR.LastIndexOf(AFTER));
        }
        public bool IsHybernationEnabled() {
            string rootDrive = System.IO.Path.GetPathRoot(Environment.SystemDirectory);
            if (System.IO.File.Exists(rootDrive + @":\hiberfil.sys")) {
                return true;
            } else {
                return true;
            }
        }
        #endregion
        #region "UX"
        public Form1() {
            InitializeComponent();
        }
        private void ShowTooltip(string STR) {
            NotifyIcon1.ShowBalloonTip(1000, "xxUSBSentinel", STR, ToolTipIcon.Info);
        }
        private void ShowGUI() {
            this.ShowInTaskbar = true;
            this.Show();
            this.Focus();
            this.WindowState = FormWindowState.Normal;
        }
        private void HideGUI() {
            this.WindowState = FormWindowState.Minimized;
            this.ShowInTaskbar = false;
            this.Hide();
        }
        private void ExitToolStripMenuItem_Click(object sender, EventArgs e) {
            Exit();
        }
        private void Form1_FormClosing(object sender, FormClosingEventArgs e) {
            e.Cancel = true;
            HideGUI();
        }
        private void ToolStripMenuItem1_Click(object sender, EventArgs e) {
            ShowGUI();
        }
        private void ClearLogToolStripMenuItem_Click(object sender, EventArgs e) {
            ListBox1.Items.Clear();
        }
        private void ToolStripMenuItem4_Click(object sender, EventArgs e) {
            ShowHelp();
        }
        private void NotifyIcon1_MouseDoubleClick(object sender, MouseEventArgs e) {
            ShowGUI();
        }
        private void CopyEventToolStripMenuItem_Click(object sender, EventArgs e) {
            string s = ListBox1.SelectedItem.ToString();
            Clipboard.SetData(DataFormats.StringFormat, s);
        }
        private void ListBox1_MouseDown(object sender, MouseEventArgs e) {
            if (e.Button == MouseButtons.Right) {
                if (ListBox1.SelectedIndex != -1) {
                    ContextMenuStrip2.Show(Cursor.Position);
                }
            } else {
                ListBox1.SelectedIndex = ListBox1.IndexFromPoint(e.Location);
            }
        }
        private void CopyVIDToolStripMenuItem_Click(object sender, EventArgs e) {
            string s = ListBox1.SelectedItem.ToString();
            VID = GetVID(s);
            Clipboard.SetData(DataFormats.StringFormat, VID);
        }
        private void CopyPIDToolStripMenuItem_Click(object sender, EventArgs e) {
            string s = ListBox1.SelectedItem.ToString();
            VID = GetPID(s);
            Clipboard.SetData(DataFormats.StringFormat, VID);
        }
        private void CopyVIDPIDToolStripMenuItem_Click(object sender, EventArgs e) {
            string s = ListBox1.SelectedItem.ToString();
            VID = GetVID(s);
            PID = GetPID(s);
            Clipboard.SetData(DataFormats.StringFormat, VID + ":" + PID);
        }
        private void Button1_Click(object sender, EventArgs e) {
            MapDevice();
        }
        private void Button2_Click(object sender, EventArgs e) {
            ToggleSentinel();
        }
        private void Button3_Click(object sender, EventArgs e) {
            ToggleTestMode();
        }

        private void ToolStripMenuItem2_Click(object sender, EventArgs e) {
            ToggleSentinel();
        }
        private void ToolStripMenuItem3_Click(object sender, EventArgs e) {
            ToggleTestMode();
        }
        private void MapDeviceToolStripMenuItem_Click(object sender, EventArgs e) {
            MapDevice();
        }
        private void Button4_Click(object sender, EventArgs e) {
            Exit();
        }
        private void Exit() {
            this.Hide();
            NotifyIcon1.Visible = false;
            Process.GetCurrentProcess().Kill();
        }
        private void ExportLogToolStripMenuItem_Click(object sender, EventArgs e) {
            ExportLog();
        }
        private void Button5_Click(object sender, EventArgs e) {
            ShowHelp();
        }
        private void Button6_Click(object sender, EventArgs e) {
            ShowAbout();
        }
        private void AboutToolStripMenuItem_Click(object sender, EventArgs e) {
            ShowAbout();
        }

        #endregion
        public string GetPID(string STR) {
            STR = STR.Split('&')[1];
            return Between(STR, "PID_", "#");
        }
        public string GetVID(string STR) {
            return Between(STR, "VID_", "&PID_");
        }
        public string GetAction(string STR) {
            if (STR.Contains("VID_")) {
                if (STR.Contains("DeviceArrival")) {
                    return CONNECTED;
                } else if (STR.Contains("DeviceRemoveComplete")) {
                    return DISCONNECTED;
                }
            }
            return "Other";
        }
        private void ExportLog() {
            string now = DateTime.Now.ToString("dd.MM.yyyy-hh-mm-ss");
            SaveFileDialog s = new SaveFileDialog();
            s.Title = "Save log file";
            s.FileName = now + "-xxUSBSentinel.log";
            s.Filter = "Text files (*.txt)|*.txt|Log files (*.log)|*.log|All files (*.*)|*.*";
            if (s.ShowDialog() == DialogResult.OK) {
                System.IO.StreamWriter SaveFile = new System.IO.StreamWriter(s.OpenFile());
                foreach (var item in ListBox1.Items) {
                    SaveFile.WriteLine(item.ToString());
                }
                MessageBox.Show("Saved!");
                SaveFile.Dispose();
            }

        }
        private void ShowHelp() {
            string HelpString = "The purpose of this software is to provide a kill switch for encrypted computers." + Environment.NewLine +
                "1. Map a Key USB device (it can be a mouse, flash drive, etc)." + Environment.NewLine +
                "2. Arm the Sentinel." + Environment.NewLine +
                "Now every time you unplug your mapped USB Key device, your PC will shutdown making encryption key recovery almost impossible." + Environment.NewLine +
                "Note: This program will not provide protection to your files, you must do it on your own (try VeraCrypt).";
            MessageBox.Show(HelpString, "xxUSBSentinel");
        }
        private void ShowAbout() {
            string AboutString = "Version: 1.0" + Environment.NewLine +
                @"Author: https://github.com/thereisnotime" + Environment.NewLine +
                "For bug reporting and more useful tools check out my github.";
            MessageBox.Show(AboutString, "xxUSBSentinel");
        }
        private void MapDevice() {
            Armed = false;
            Button1.Enabled = false;
            Button1.Text = "Unplug desired USB";
            MapDeviceToolStripMenuItem.Enabled = false;
            MapDeviceToolStripMenuItem.Text = "Unplug desired USB";
            Waiting = true;
            ShowTooltip("Now unplug the desired USB");
        }
        private void ToggleSentinel() {
            if (Armed) {
                PictureBox1.Image = Properties.Resources.guard_off;
                NotifyIcon1.Icon = Properties.Resources.guard_off1;
                NotifyIcon1.Visible = true;
                Button2.Text = "Arm Sentinel";
                toolStripMenuItem2.Text = "Arm Sentinel";
                Armed = false;
                Button1.Enabled = true;
                toolStripMenuItem2.Enabled = true;

            } else {
                PictureBox1.Image = Properties.Resources.guard_on;
                NotifyIcon1.Icon = Properties.Resources.guard_on1;
                NotifyIcon1.Visible = true;
                Button2.Text = "Disarm Sentinel";
                toolStripMenuItem2.Text = "Disarm Sentinel";
                Armed = true;
                Button1.Enabled = false;
            }
        }
        private void ResolveDeviceToolStripMenuItem_Click(object sender, EventArgs e) {
            string s = ListBox1.SelectedItem.ToString();
            VID = GetVID(s);
            PID = GetPID(s);
            Process.Start(@"https://www.the-sz.com/products/usbid/index.php?v=" + VID + "&p=" + PID + "&n=");
        }
        private void ToggleTestMode() {
            if (TestMode) {
                Button3.Text = "Enable Test Mode";
                toolStripMenuItem3.Text = "Enable Test Mode";
                TestMode = false;
            } else {
                Button3.Text = "Disable Test Mode";
                toolStripMenuItem3.Text = "Disable Test Mode";
                TestMode = true;
            }
        }
        private void LogEvent(string STR) {
            string now = DateTime.Now.ToString("dd/MM/yyyy hh:mm:ss");
            ListBox1.Items.Add(now + ": " + STR);
        }
        private void SleepTightSweetPrince() {
            var psi = new ProcessStartInfo("shutdown", "/s /t 0 /f");
            psi.CreateNoWindow = true;
            psi.UseShellExecute = false;
            Process.Start(psi);
        }
        private void TestAction() {
            MessageBox.Show("Good thing this is just a test.");
        }
        private void OnDeviceNotifyEvent(object sender, DeviceNotifyEventArgs e) {
            Data = e.ToString();
            Action = GetAction(Data);
            LogEvent(Data);
            if (Debug) {
                Console.WriteLine(Data);
                if (Action == CONNECTED || Action == DISCONNECTED) {
                    VID = GetVID(Data);
                    PID = GetPID(Data);
                } else {
                    if (Debug) Console.WriteLine("Other USB Event.");
                }
            }
            if (Waiting) {
                if (Action == DISCONNECTED) {
                    VID = GetVID(Data);
                    PID = GetPID(Data);
                    string currentDevice = VID + ":" + PID;
                    KeyDevice = currentDevice;
                    TextBox1.Text = currentDevice;
                    ToolStripTextBox1.Text = currentDevice;
                    Button1.Enabled = true;
                    Button1.Text = "Map USB Device";
                    MapDeviceToolStripMenuItem.Enabled = true;
                    MapDeviceToolStripMenuItem.Text = "Map USB Device";
                    Waiting = false;
                    Button2.Enabled = true;
                    toolStripMenuItem2.Enabled = true;
                    NotifyIcon1.Visible = true;
                    ShowTooltip("Device mapped. You can arm the Sentinel.");
                }
            }
            if (Action == DISCONNECTED) {
                VID = GetVID(Data);
                PID = GetPID(Data);
                string currentDevice = VID + ":" + PID;
                Console.WriteLine("Current Device: " + currentDevice);
                if (currentDevice == KeyDevice) {
                    if (Armed) {
                        if (TestMode) {
                            TestAction();
                        } else {
                            SleepTightSweetPrince();
                        }
                    }
                }
            }
        }
        private void Form1_Load(object sender, EventArgs e) {
            UsbDeviceNotifier.OnDeviceNotify += OnDeviceNotifyEvent;
        }
    }
}
