@0x9ec2186157e93118;

struct Status {
    version @0: Int32;
    networkid @1: Text;
    port @2: Int32;
    guid @3: Text;
    publicPort @4: Int32;
}

interface Network {
    getStatus @0 () -> (status :Status);
}
