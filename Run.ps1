param (
    [int]$iterations = 100000000,
    [string]$numbers = "1 | 2 | 3 | 4|5|6|7|8|9|10|11"
)
function RunCs([hashtable] $times) {
    dotnet build -c Release --nologo --verbosity quiet .\ParseCs\ParseCs\ParseCs.csproj

    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    .\ParseCs\ParseCs\bin\Release\net5.0\ParseCs.exe $iterations "$numbers"
    $sw.Stop()

    $times.Add("C#", $sw.Elapsed.TotalSeconds)
}



function RunRust([hashtable] $times) {
    Set-Location .\ParseRust\
    cargo build --release

    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    .\target\release\parse_rust.exe $iterations "$numbers"
    $sw.Stop()
    
    Set-Location ..

    $times.Add("Rust", $sw.Elapsed.TotalSeconds)
}


function RunJava([hashtable] $times) {
    Set-Location .\ParseJava\
    javac -d .\target\ .\src\main\java\simpleParse\App.java

    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    java.exe -cp '.\target\' 'simpleParse.App' $iterations "$numbers"
    $sw.Stop()
    $times.Add("Java", $sw.Elapsed.TotalSeconds)
   
   
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    java.exe -cp '.\target\' 'simpleParse.App' $iterations "$numbers" "--stream"
    $sw.Stop()
    $times.Add("Java(streams)", $sw.Elapsed.TotalSeconds)

    Set-Location ..   
}

Clear-Host
[hashtable] $times = @{}
RunCs($times)
RunRust($times)
RunJava($times)

$times | Format-Table Name, @{Label = "Time"; Expression = { $_.Value.ToString("0.####") } }

