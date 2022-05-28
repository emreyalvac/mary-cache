const grpc = require("k6/net/grpc");
const client = new grpc.Client();

client.load(['protos'], 'cache.proto');

export const options = {
    insecureSkipTLSVerify: true,
};

export default () => {
    client.connect('0.0.0.0:3000', {
        plaintext: false,
        reflect: true
    });
    const data = {key: 'Key', value: "Value"};
    const response = client.invoke('cache.CacheService/Set', data);
    check(response, {
        'status is OK': (r) => r && r.status === grpc.StatusOK,
    });

    console.log(JSON.stringify(response.message));
    client.close();
    sleep(1);
};