[Setup]
AppName=tpmgr
AppVersion=0.1.0
AppVerName=tpmgr
AppPublisher=tpmgr Contributors
AppPublisherURL=https://github.com/jiaojiaodubai/tpmgr
AppSupportURL=https://github.com/jiaojiaodubai/tpmgr/issues
AppUpdatesURL=https://github.com/jiaojiaodubai/tpmgr/releases
DefaultDirName={localappdata}\tpmgr
DisableProgramGroupPage=yes
LicenseFile=..\..\LICENSE
OutputDir=..\..\dist
OutputBaseFilename=tpmgr-0.1.0-setup
Compression=lzma
SolidCompression=yes
PrivilegesRequired=lowest
ShowLanguageDialog=yes
LanguageDetectionMethod=uilanguage

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "chinesesimplified"; MessagesFile: "ChineseSimplified.isl"
Name: "chinesetraditional"; MessagesFile: "ChineseTraditional.isl"

[CustomMessages]
; English messages
english.WelcomeLabel2=This will install [name] on your computer.%n%ntpmgr is a modern LaTeX package manager inspired by Python's uv and TeXLive's tlmgr.%n%nIt is recommended that you close all other applications before continuing.
english.FinishedLabel=Setup has finished installing [name] on your computer.%n%nThe application has been added to your PATH environment variable. You can now use the 'tpmgr' command in any command prompt.%n%nPlease restart your terminal to start using tpmgr!
english.AddingToPath=Adding to PATH environment variable...
english.RemovingFromPath=Removing from PATH environment variable...
english.ComponentMain=Core tpmgr application
english.ComponentPath=Add tpmgr to PATH environment variable
english.MainComponent=tpmgr Core Application
english.AddPath=Add tpmgr to PATH environment variable

; Chinese Simplified messages  
chinesesimplified.WelcomeLabel2=此程序将在您的计算机上安装 [name]。%n%ntpmgr 是一个现代化的 LaTeX 包管理器，灵感来源于 Python 的 uv 和 TeXLive 的 tlmgr。%n%n建议您在继续之前关闭所有其他应用程序。
chinesesimplified.FinishedLabel=安装程序已完成在您的计算机上安装 [name]。%n%n应用程序已添加到您的 PATH 环境变量中。您现在可以在任何命令提示符中使用 'tpmgr' 命令。%n%n请重新启动您的终端以开始使用 tpmgr！
chinesesimplified.AddingToPath=正在添加到 PATH 环境变量...
chinesesimplified.RemovingFromPath=正在从 PATH 环境变量中移除...
chinesesimplified.ComponentMain=tpmgr 核心应用程序
chinesesimplified.ComponentPath=将 tpmgr 添加到 PATH 环境变量
chinesesimplified.MainComponent=tpmgr 核心应用程序
chinesesimplified.AddPath=将 tpmgr 添加到 PATH 环境变量

; Chinese Traditional messages
chinesetraditional.WelcomeLabel2=此程式將在您的電腦上安裝 [name]。%n%ntpmgr 是一個現代化的 LaTeX 套件管理器，靈感來源於 Python 的 uv 和 TeXLive 的 tlmgr。%n%n建議您在繼續之前關閉所有其他應用程式。
chinesetraditional.FinishedLabel=安裝程式已完成在您的電腦上安裝 [name]。%n%n應用程式已新增到您的 PATH 環境變數中。您現在可以在任何命令提示字元中使用 'tpmgr' 命令。%n%n請重新啟動您的終端以開始使用 tpmgr！
chinesetraditional.AddingToPath=正在新增到 PATH 環境變數...
chinesetraditional.RemovingFromPath=正在從 PATH 環境變數中移除...
chinesetraditional.ComponentMain=tpmgr 核心應用程式
chinesetraditional.ComponentPath=將 tpmgr 新增到 PATH 環境變數
chinesetraditional.MainComponent=tpmgr 核心應用程式
chinesetraditional.AddPath=將 tpmgr 新增到 PATH 環境變數

[Types]
Name: "full"; Description: "{cm:ComponentMain}"
Name: "compact"; Description: "{cm:ComponentMain}"
Name: "custom"; Description: "{cm:ComponentMain}"; Flags: iscustom

[Components]
Name: "main"; Description: "{cm:MainComponent}"; Types: full compact custom; Flags: fixed
Name: "addpath"; Description: "{cm:AddPath}"; Types: full

[Files]
Source: "..\..\target\release\tpmgr.exe"; DestDir: "{app}"; Flags: ignoreversion; Components: main

[Code]
function AddToPath(Dir: string): Boolean;
var
  Path: string;
  NewPath: string;
begin
  Result := False;
  
  try
    if RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'PATH', Path) then
    begin
      if Pos(';' + Dir + ';', ';' + Path + ';') = 0 then
      begin
        if Path <> '' then
          NewPath := Path + ';' + Dir
        else
          NewPath := Dir;
          
        if RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'PATH', NewPath) then
        begin
          Result := True;
        end;
      end
      else
      begin
        Result := True;
      end;
    end
    else
    begin
      if RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'PATH', Dir) then
      begin
        Result := True;
      end;
    end;
  except
  end;
end;

function RemoveFromPath(Dir: string): Boolean;
var
  Path: string;
  NewPath: string;
  P: Integer;
begin
  Result := False;
  
  try
    if RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'PATH', Path) then
    begin
      NewPath := Path;
      
      // Remove ;Dir; pattern
      P := Pos(';' + Dir + ';', ';' + NewPath + ';');
      if P > 0 then
      begin
        Delete(NewPath, P + Length(Dir), Length(Dir) + 1);
      end
      else if Pos(Dir + ';', NewPath) = 1 then
      begin
        // Remove Dir; at the beginning
        NewPath := Copy(NewPath, Length(Dir) + 2, Length(NewPath));
      end
      else if Copy(NewPath, Length(NewPath) - Length(Dir) + 1, Length(Dir)) = Dir then
      begin
        // Remove ;Dir at the end
        if Copy(NewPath, Length(NewPath) - Length(Dir), 1) = ';' then
          NewPath := Copy(NewPath, 1, Length(NewPath) - Length(Dir) - 1)
        else
          NewPath := Copy(NewPath, 1, Length(NewPath) - Length(Dir));
      end
      else if NewPath = Dir then
      begin
        // Exact match
        NewPath := '';
      end;
      
      if RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'PATH', NewPath) then
      begin
        Result := True;
      end;
    end;
  except
  end;
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  case CurStep of
    ssPostInstall:
      begin
        if WizardIsComponentSelected('addpath') then
        begin
          AddToPath(ExpandConstant('{app}'));
        end;
      end;
  end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  case CurUninstallStep of
    usUninstall:
      begin
        RemoveFromPath(ExpandConstant('{app}'));
      end;
  end;
end;
