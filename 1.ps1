  $ErrorActionPreference = "Stop"

  $id = [guid]::NewGuid().ToString("N").Substring(0, 8)
  $user = "lsb_spike_$id"
  $share = "lsb-spike-$id"
  $dir = Join-Path $env:TEMP $share
  $passText = "Aa3!${id}Z9q"
  $pass = ConvertTo-SecureString $passText -AsPlainText -Force
  $principal = "$env:COMPUTERNAME\$user"
  $usersGroup = ((New-Object Security.Principal.SecurityIdentifier "S-1-5-32-545").Translate([Security.Principal.NTAccount]).Value -split "\\")[-1]

  try {
    New-Item -ItemType Directory -Force $dir | Out-Null
    Set-Content -LiteralPath (Join-Path $dir "input.txt") -Value "host-file"

    New-LocalUser -Name $user -Password $pass -PasswordNeverExpires -UserMayNotChangePassword
    Add-LocalGroupMember -Group $usersGroup -Member $principal
    icacls $dir /grant "${principal}:(OI)(CI)RX" | Out-Host
    New-SmbShare -Name $share -Path $dir -ReadAccess $principal | Out-Null

    foreach ($server in @("localhost", $env:COMPUTERNAME)) {
      cmd.exe /c "net use \\$server\$share /delete /y >nul 2>nul"
      cmd.exe /c "net use \\$server\$share /user:$principal $passText"
      if ($LASTEXITCODE -ne 0) { throw "net use failed for \\$server\$share" }

      cmd.exe /c "type \\$server\$share\input.txt"
      if ($LASTEXITCODE -ne 0) { throw "read failed for \\$server\$share" }

      cmd.exe /c "net use \\$server\$share /delete /y"
    }
  } finally {
    Remove-SmbShare -Name $share -Force -ErrorAction SilentlyContinue
    Remove-LocalUser -Name $user -ErrorAction SilentlyContinue
    Remove-Item -LiteralPath $dir -Recurse -Force -ErrorAction SilentlyContinue
  }
