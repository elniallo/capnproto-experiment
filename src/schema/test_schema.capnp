@0x9ec2186157e93118;

interface Status {
    version @0()-> (version :Int32);
    networkid @1()-> (networkid :Text);
    port @2()-> (port :Int32);
    guid @3()-> (guid :Text);
    publicPort @4()-> (publicport :Int32);
}
interface Network {
    getStatus @0 () -> (status :Status);
}
