namespace xxUSBSentinel {
    partial class Form1 {
        /// <summary>
        /// Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing) {
            if (disposing && (components != null)) {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent() {
            this.components = new System.ComponentModel.Container();
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(Form1));
            this.NotifyIcon1 = new System.Windows.Forms.NotifyIcon(this.components);
            this.ContextMenuStrip1 = new System.Windows.Forms.ContextMenuStrip(this.components);
            this.toolStripMenuItem1 = new System.Windows.Forms.ToolStripMenuItem();
            this.toolStripSeparator4 = new System.Windows.Forms.ToolStripSeparator();
            this.ToolStripTextBox1 = new System.Windows.Forms.ToolStripTextBox();
            this.MapDeviceToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.toolStripSeparator2 = new System.Windows.Forms.ToolStripSeparator();
            this.toolStripMenuItem2 = new System.Windows.Forms.ToolStripMenuItem();
            this.toolStripMenuItem3 = new System.Windows.Forms.ToolStripMenuItem();
            this.toolStripSeparator1 = new System.Windows.Forms.ToolStripSeparator();
            this.toolStripMenuItem4 = new System.Windows.Forms.ToolStripMenuItem();
            this.exitToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.groupBox1 = new System.Windows.Forms.GroupBox();
            this.ListBox1 = new System.Windows.Forms.ListBox();
            this.ContextMenuStrip2 = new System.Windows.Forms.ContextMenuStrip(this.components);
            this.copyEventToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.copyVIDToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.copyPIDToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.copyVIDPIDToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.resolveDeviceToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.toolStripSeparator3 = new System.Windows.Forms.ToolStripSeparator();
            this.clearLogToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.exportLogToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.groupBox2 = new System.Windows.Forms.GroupBox();
            this.button4 = new System.Windows.Forms.Button();
            this.label1 = new System.Windows.Forms.Label();
            this.TextBox1 = new System.Windows.Forms.TextBox();
            this.Button3 = new System.Windows.Forms.Button();
            this.Button1 = new System.Windows.Forms.Button();
            this.Button2 = new System.Windows.Forms.Button();
            this.PictureBox1 = new System.Windows.Forms.PictureBox();
            this.Button5 = new System.Windows.Forms.Button();
            this.Button6 = new System.Windows.Forms.Button();
            this.AboutToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.ContextMenuStrip1.SuspendLayout();
            this.groupBox1.SuspendLayout();
            this.ContextMenuStrip2.SuspendLayout();
            this.groupBox2.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.PictureBox1)).BeginInit();
            this.SuspendLayout();
            // 
            // NotifyIcon1
            // 
            this.NotifyIcon1.BalloonTipIcon = System.Windows.Forms.ToolTipIcon.Info;
            this.NotifyIcon1.BalloonTipText = "To serve and protect.";
            this.NotifyIcon1.BalloonTipTitle = "xxUSBSentinel";
            this.NotifyIcon1.ContextMenuStrip = this.ContextMenuStrip1;
            this.NotifyIcon1.Icon = ((System.Drawing.Icon)(resources.GetObject("NotifyIcon1.Icon")));
            this.NotifyIcon1.Text = "xxUSBSentinel";
            this.NotifyIcon1.Visible = true;
            this.NotifyIcon1.DoubleClick += new System.EventHandler(this.ToolStripMenuItem1_Click);
            this.NotifyIcon1.MouseDoubleClick += new System.Windows.Forms.MouseEventHandler(this.NotifyIcon1_MouseDoubleClick);
            // 
            // ContextMenuStrip1
            // 
            this.ContextMenuStrip1.ImageScalingSize = new System.Drawing.Size(24, 24);
            this.ContextMenuStrip1.Items.AddRange(new System.Windows.Forms.ToolStripItem[] {
            this.toolStripMenuItem1,
            this.toolStripSeparator4,
            this.ToolStripTextBox1,
            this.MapDeviceToolStripMenuItem,
            this.toolStripSeparator2,
            this.toolStripMenuItem2,
            this.toolStripMenuItem3,
            this.toolStripSeparator1,
            this.toolStripMenuItem4,
            this.AboutToolStripMenuItem,
            this.exitToolStripMenuItem});
            this.ContextMenuStrip1.Name = "contextMenuStrip1";
            this.ContextMenuStrip1.Size = new System.Drawing.Size(241, 298);
            // 
            // toolStripMenuItem1
            // 
            this.toolStripMenuItem1.Name = "toolStripMenuItem1";
            this.toolStripMenuItem1.Size = new System.Drawing.Size(240, 30);
            this.toolStripMenuItem1.Text = "Show GUI";
            this.toolStripMenuItem1.Click += new System.EventHandler(this.ToolStripMenuItem1_Click);
            // 
            // toolStripSeparator4
            // 
            this.toolStripSeparator4.Name = "toolStripSeparator4";
            this.toolStripSeparator4.Size = new System.Drawing.Size(237, 6);
            // 
            // ToolStripTextBox1
            // 
            this.ToolStripTextBox1.Name = "ToolStripTextBox1";
            this.ToolStripTextBox1.ReadOnly = true;
            this.ToolStripTextBox1.Size = new System.Drawing.Size(100, 31);
            this.ToolStripTextBox1.Text = "NO DEVICE";
            this.ToolStripTextBox1.TextBoxTextAlign = System.Windows.Forms.HorizontalAlignment.Center;
            // 
            // MapDeviceToolStripMenuItem
            // 
            this.MapDeviceToolStripMenuItem.Name = "MapDeviceToolStripMenuItem";
            this.MapDeviceToolStripMenuItem.Size = new System.Drawing.Size(240, 30);
            this.MapDeviceToolStripMenuItem.Text = "Map Device";
            this.MapDeviceToolStripMenuItem.Click += new System.EventHandler(this.MapDeviceToolStripMenuItem_Click);
            // 
            // toolStripSeparator2
            // 
            this.toolStripSeparator2.Name = "toolStripSeparator2";
            this.toolStripSeparator2.Size = new System.Drawing.Size(237, 6);
            // 
            // toolStripMenuItem2
            // 
            this.toolStripMenuItem2.Enabled = false;
            this.toolStripMenuItem2.Name = "toolStripMenuItem2";
            this.toolStripMenuItem2.Size = new System.Drawing.Size(240, 30);
            this.toolStripMenuItem2.Text = "Arm Sentinel";
            this.toolStripMenuItem2.Click += new System.EventHandler(this.ToolStripMenuItem2_Click);
            // 
            // toolStripMenuItem3
            // 
            this.toolStripMenuItem3.Name = "toolStripMenuItem3";
            this.toolStripMenuItem3.Size = new System.Drawing.Size(240, 30);
            this.toolStripMenuItem3.Text = "Enable Test Mode";
            this.toolStripMenuItem3.Click += new System.EventHandler(this.ToolStripMenuItem3_Click);
            // 
            // toolStripSeparator1
            // 
            this.toolStripSeparator1.Name = "toolStripSeparator1";
            this.toolStripSeparator1.Size = new System.Drawing.Size(237, 6);
            // 
            // toolStripMenuItem4
            // 
            this.toolStripMenuItem4.Name = "toolStripMenuItem4";
            this.toolStripMenuItem4.Size = new System.Drawing.Size(240, 30);
            this.toolStripMenuItem4.Text = "Help";
            this.toolStripMenuItem4.Click += new System.EventHandler(this.ToolStripMenuItem4_Click);
            // 
            // exitToolStripMenuItem
            // 
            this.exitToolStripMenuItem.Name = "exitToolStripMenuItem";
            this.exitToolStripMenuItem.Size = new System.Drawing.Size(240, 30);
            this.exitToolStripMenuItem.Text = "Exit";
            this.exitToolStripMenuItem.Click += new System.EventHandler(this.ExitToolStripMenuItem_Click);
            // 
            // groupBox1
            // 
            this.groupBox1.Controls.Add(this.ListBox1);
            this.groupBox1.Location = new System.Drawing.Point(12, 12);
            this.groupBox1.Name = "groupBox1";
            this.groupBox1.Size = new System.Drawing.Size(776, 196);
            this.groupBox1.TabIndex = 1;
            this.groupBox1.TabStop = false;
            this.groupBox1.Text = "Device Logger";
            // 
            // ListBox1
            // 
            this.ListBox1.FormattingEnabled = true;
            this.ListBox1.HorizontalScrollbar = true;
            this.ListBox1.ItemHeight = 20;
            this.ListBox1.Location = new System.Drawing.Point(12, 34);
            this.ListBox1.Name = "ListBox1";
            this.ListBox1.ScrollAlwaysVisible = true;
            this.ListBox1.Size = new System.Drawing.Size(745, 144);
            this.ListBox1.TabIndex = 2;
            this.ListBox1.MouseDown += new System.Windows.Forms.MouseEventHandler(this.ListBox1_MouseDown);
            // 
            // ContextMenuStrip2
            // 
            this.ContextMenuStrip2.ImageScalingSize = new System.Drawing.Size(24, 24);
            this.ContextMenuStrip2.Items.AddRange(new System.Windows.Forms.ToolStripItem[] {
            this.copyEventToolStripMenuItem,
            this.copyVIDToolStripMenuItem,
            this.copyPIDToolStripMenuItem,
            this.copyVIDPIDToolStripMenuItem,
            this.resolveDeviceToolStripMenuItem,
            this.toolStripSeparator3,
            this.clearLogToolStripMenuItem,
            this.exportLogToolStripMenuItem});
            this.ContextMenuStrip2.Name = "contextMenuStrip2";
            this.ContextMenuStrip2.Size = new System.Drawing.Size(258, 220);
            // 
            // copyEventToolStripMenuItem
            // 
            this.copyEventToolStripMenuItem.Name = "copyEventToolStripMenuItem";
            this.copyEventToolStripMenuItem.Size = new System.Drawing.Size(257, 30);
            this.copyEventToolStripMenuItem.Text = "Copy Event";
            this.copyEventToolStripMenuItem.Click += new System.EventHandler(this.CopyEventToolStripMenuItem_Click);
            // 
            // copyVIDToolStripMenuItem
            // 
            this.copyVIDToolStripMenuItem.Name = "copyVIDToolStripMenuItem";
            this.copyVIDToolStripMenuItem.Size = new System.Drawing.Size(257, 30);
            this.copyVIDToolStripMenuItem.Text = "Copy VID";
            this.copyVIDToolStripMenuItem.Click += new System.EventHandler(this.CopyVIDToolStripMenuItem_Click);
            // 
            // copyPIDToolStripMenuItem
            // 
            this.copyPIDToolStripMenuItem.Name = "copyPIDToolStripMenuItem";
            this.copyPIDToolStripMenuItem.Size = new System.Drawing.Size(257, 30);
            this.copyPIDToolStripMenuItem.Text = "Copy PID";
            this.copyPIDToolStripMenuItem.Click += new System.EventHandler(this.CopyPIDToolStripMenuItem_Click);
            // 
            // copyVIDPIDToolStripMenuItem
            // 
            this.copyVIDPIDToolStripMenuItem.Name = "copyVIDPIDToolStripMenuItem";
            this.copyVIDPIDToolStripMenuItem.Size = new System.Drawing.Size(257, 30);
            this.copyVIDPIDToolStripMenuItem.Text = "Copy VID:PID";
            this.copyVIDPIDToolStripMenuItem.Click += new System.EventHandler(this.CopyVIDPIDToolStripMenuItem_Click);
            // 
            // resolveDeviceToolStripMenuItem
            // 
            this.resolveDeviceToolStripMenuItem.Name = "resolveDeviceToolStripMenuItem";
            this.resolveDeviceToolStripMenuItem.Size = new System.Drawing.Size(257, 30);
            this.resolveDeviceToolStripMenuItem.Text = "Resolve Device Online";
            this.resolveDeviceToolStripMenuItem.Click += new System.EventHandler(this.ResolveDeviceToolStripMenuItem_Click);
            // 
            // toolStripSeparator3
            // 
            this.toolStripSeparator3.Name = "toolStripSeparator3";
            this.toolStripSeparator3.Size = new System.Drawing.Size(254, 6);
            // 
            // clearLogToolStripMenuItem
            // 
            this.clearLogToolStripMenuItem.Name = "clearLogToolStripMenuItem";
            this.clearLogToolStripMenuItem.Size = new System.Drawing.Size(257, 30);
            this.clearLogToolStripMenuItem.Text = "Clear Log";
            this.clearLogToolStripMenuItem.Click += new System.EventHandler(this.ClearLogToolStripMenuItem_Click);
            // 
            // exportLogToolStripMenuItem
            // 
            this.exportLogToolStripMenuItem.Name = "exportLogToolStripMenuItem";
            this.exportLogToolStripMenuItem.Size = new System.Drawing.Size(257, 30);
            this.exportLogToolStripMenuItem.Text = "Export Log";
            this.exportLogToolStripMenuItem.Click += new System.EventHandler(this.ExportLogToolStripMenuItem_Click);
            // 
            // groupBox2
            // 
            this.groupBox2.Controls.Add(this.button4);
            this.groupBox2.Controls.Add(this.label1);
            this.groupBox2.Controls.Add(this.TextBox1);
            this.groupBox2.Controls.Add(this.Button3);
            this.groupBox2.Controls.Add(this.Button1);
            this.groupBox2.Location = new System.Drawing.Point(12, 226);
            this.groupBox2.Name = "groupBox2";
            this.groupBox2.Size = new System.Drawing.Size(245, 229);
            this.groupBox2.TabIndex = 2;
            this.groupBox2.TabStop = false;
            this.groupBox2.Text = "Main Controls";
            // 
            // button4
            // 
            this.button4.Location = new System.Drawing.Point(12, 179);
            this.button4.Name = "button4";
            this.button4.Size = new System.Drawing.Size(227, 44);
            this.button4.TabIndex = 3;
            this.button4.Text = "Exit";
            this.button4.UseVisualStyleBackColor = true;
            this.button4.Click += new System.EventHandler(this.Button4_Click);
            // 
            // label1
            // 
            this.label1.AutoSize = true;
            this.label1.Location = new System.Drawing.Point(18, 45);
            this.label1.Name = "label1";
            this.label1.Size = new System.Drawing.Size(68, 20);
            this.label1.TabIndex = 2;
            this.label1.Text = "VID:PID";
            // 
            // TextBox1
            // 
            this.TextBox1.Location = new System.Drawing.Point(92, 42);
            this.TextBox1.Name = "TextBox1";
            this.TextBox1.ReadOnly = true;
            this.TextBox1.Size = new System.Drawing.Size(147, 26);
            this.TextBox1.TabIndex = 1;
            this.TextBox1.Text = "NO DEVICE";
            this.TextBox1.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
            // 
            // Button3
            // 
            this.Button3.Location = new System.Drawing.Point(12, 129);
            this.Button3.Name = "Button3";
            this.Button3.Size = new System.Drawing.Size(227, 44);
            this.Button3.TabIndex = 4;
            this.Button3.Text = "Enable Test Mode";
            this.Button3.UseVisualStyleBackColor = true;
            this.Button3.Click += new System.EventHandler(this.Button3_Click);
            // 
            // Button1
            // 
            this.Button1.Location = new System.Drawing.Point(12, 75);
            this.Button1.Name = "Button1";
            this.Button1.Size = new System.Drawing.Size(227, 48);
            this.Button1.TabIndex = 0;
            this.Button1.Text = "Map USB Device";
            this.Button1.UseVisualStyleBackColor = true;
            this.Button1.Click += new System.EventHandler(this.Button1_Click);
            // 
            // Button2
            // 
            this.Button2.Enabled = false;
            this.Button2.Location = new System.Drawing.Point(508, 226);
            this.Button2.Name = "Button2";
            this.Button2.Size = new System.Drawing.Size(280, 173);
            this.Button2.TabIndex = 3;
            this.Button2.Text = "Arm Sentinel";
            this.Button2.UseVisualStyleBackColor = true;
            this.Button2.Click += new System.EventHandler(this.Button2_Click);
            // 
            // PictureBox1
            // 
            this.PictureBox1.Image = ((System.Drawing.Image)(resources.GetObject("PictureBox1.Image")));
            this.PictureBox1.Location = new System.Drawing.Point(283, 255);
            this.PictureBox1.Name = "PictureBox1";
            this.PictureBox1.Size = new System.Drawing.Size(200, 200);
            this.PictureBox1.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage;
            this.PictureBox1.TabIndex = 5;
            this.PictureBox1.TabStop = false;
            // 
            // Button5
            // 
            this.Button5.Location = new System.Drawing.Point(508, 405);
            this.Button5.Name = "Button5";
            this.Button5.Size = new System.Drawing.Size(183, 50);
            this.Button5.TabIndex = 6;
            this.Button5.Text = "Help";
            this.Button5.UseVisualStyleBackColor = true;
            this.Button5.Click += new System.EventHandler(this.Button5_Click);
            // 
            // Button6
            // 
            this.Button6.Location = new System.Drawing.Point(697, 405);
            this.Button6.Name = "Button6";
            this.Button6.Size = new System.Drawing.Size(91, 50);
            this.Button6.TabIndex = 7;
            this.Button6.Text = "About";
            this.Button6.UseVisualStyleBackColor = true;
            this.Button6.Click += new System.EventHandler(this.Button6_Click);
            // 
            // AboutToolStripMenuItem
            // 
            this.AboutToolStripMenuItem.Name = "AboutToolStripMenuItem";
            this.AboutToolStripMenuItem.Size = new System.Drawing.Size(240, 30);
            this.AboutToolStripMenuItem.Text = "About";
            this.AboutToolStripMenuItem.Click += new System.EventHandler(this.AboutToolStripMenuItem_Click);
            // 
            // Form1
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(9F, 20F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(800, 467);
            this.Controls.Add(this.Button6);
            this.Controls.Add(this.Button5);
            this.Controls.Add(this.PictureBox1);
            this.Controls.Add(this.Button2);
            this.Controls.Add(this.groupBox2);
            this.Controls.Add(this.groupBox1);
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedSingle;
            this.Icon = ((System.Drawing.Icon)(resources.GetObject("$this.Icon")));
            this.MaximizeBox = false;
            this.MinimizeBox = false;
            this.Name = "Form1";
            this.Text = "xxUSBSentinel";
            this.FormClosing += new System.Windows.Forms.FormClosingEventHandler(this.Form1_FormClosing);
            this.Load += new System.EventHandler(this.Form1_Load);
            this.ContextMenuStrip1.ResumeLayout(false);
            this.ContextMenuStrip1.PerformLayout();
            this.groupBox1.ResumeLayout(false);
            this.ContextMenuStrip2.ResumeLayout(false);
            this.groupBox2.ResumeLayout(false);
            this.groupBox2.PerformLayout();
            ((System.ComponentModel.ISupportInitialize)(this.PictureBox1)).EndInit();
            this.ResumeLayout(false);

        }

        #endregion

        private System.Windows.Forms.NotifyIcon NotifyIcon1;
        private System.Windows.Forms.ContextMenuStrip ContextMenuStrip1;
        private System.Windows.Forms.ToolStripMenuItem toolStripMenuItem1;
        private System.Windows.Forms.ToolStripMenuItem toolStripMenuItem2;
        private System.Windows.Forms.ToolStripMenuItem toolStripMenuItem3;
        private System.Windows.Forms.ToolStripMenuItem exitToolStripMenuItem;
        private System.Windows.Forms.ToolStripMenuItem MapDeviceToolStripMenuItem;
        private System.Windows.Forms.ToolStripSeparator toolStripSeparator2;
        private System.Windows.Forms.ToolStripSeparator toolStripSeparator1;
        private System.Windows.Forms.GroupBox groupBox1;
        private System.Windows.Forms.ListBox ListBox1;
        private System.Windows.Forms.ContextMenuStrip ContextMenuStrip2;
        private System.Windows.Forms.ToolStripMenuItem copyEventToolStripMenuItem;
        private System.Windows.Forms.ToolStripMenuItem copyVIDToolStripMenuItem;
        private System.Windows.Forms.ToolStripMenuItem copyPIDToolStripMenuItem;
        private System.Windows.Forms.ToolStripMenuItem copyVIDPIDToolStripMenuItem;
        private System.Windows.Forms.ToolStripMenuItem clearLogToolStripMenuItem;
        private System.Windows.Forms.GroupBox groupBox2;
        private System.Windows.Forms.Label label1;
        private System.Windows.Forms.TextBox TextBox1;
        private System.Windows.Forms.Button Button1;
        private System.Windows.Forms.Button Button2;
        private System.Windows.Forms.Button Button3;
        private System.Windows.Forms.PictureBox PictureBox1;
        private System.Windows.Forms.Button button4;
        private System.Windows.Forms.ToolStripSeparator toolStripSeparator3;
        private System.Windows.Forms.ToolStripMenuItem exportLogToolStripMenuItem;
        private System.Windows.Forms.ToolStripSeparator toolStripSeparator4;
        private System.Windows.Forms.ToolStripTextBox ToolStripTextBox1;
        private System.Windows.Forms.Button Button5;
        private System.Windows.Forms.ToolStripMenuItem toolStripMenuItem4;
        private System.Windows.Forms.ToolStripMenuItem resolveDeviceToolStripMenuItem;
        private System.Windows.Forms.Button Button6;
        private System.Windows.Forms.ToolStripMenuItem AboutToolStripMenuItem;
    }
}

