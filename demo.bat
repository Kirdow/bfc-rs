@ECHO ^>^> Building BFC-RS ^<^<
@cargo build

@pushd %~dp0\vendor\bf-rs\
@ECHO ^>^> Building BF-RS ^<^<
@cargo build
@popd

@ECHO ^>^> Compiling demo.bfc to demo.bf ^<^<
@target\debug\bfc.exe demo/demo.bfc demo/out/demo.bf

@pushd %~dp0\vendor\bf-rs\
@ECHO ^>^> Running compiled demo.bf ^<^<
@target\debug\bf.exe ../../demo/out/demo.bf
@popd