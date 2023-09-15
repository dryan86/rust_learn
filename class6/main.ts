import { WsProvider, ApiPromise, Keyring } from "@polkadot/api";
import '@polkadot/api-augment'
import '@polkadot/types'

const sleep = (ms: number) => new Promise(r => setTimeout(r, ms));

const WEB_SOCKET = "ws://127.0.0.1:9944";
const connect = async() => {
    const wsProvider = new WsProvider(WEB_SOCKET);
    const api = await ApiPromise.create({provider: wsProvider, types:{}});
    await api.isReady;
    return api;
}

const subscribe = async(api: ApiPromise)=>{
    await api.query.system.events(events=>{
        events.forEach(event => {
            if (event['event']['index'].toHuman() == "0x0700"){
                const data = event['event']['data'].toHuman();
                console.log("SomethingStored, current value is:", data);
            }
        });
    })
}

const get_meta_info =async (api: ApiPromise) => {
    const metainfo = await api.rpc.state.getMetadata();
    return metainfo;
}

// const getsomething = async(api: ApiPromise)=>{
//     await api.query.
// }

const main = async() => {
    const api = await connect();
    const keyring = new Keyring({type: "sr25519"});
    const alice = keyring.addFromUri("//Alice");

    await subscribe(api);

    await sleep(50000);

    console.log("main function exit");
}

main().then(function(){
    console.log("sucess")
})