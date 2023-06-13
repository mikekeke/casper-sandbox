import {
  DeployUtil,
  CLPublicKey,
  EventStream,
  EventName,
  CLValueParsers,
  CLTypeTag,
  CLMap,
  CLValue,
  CLValueBuilder,
  CasperServiceByJsonRPC
} from "casper-js-sdk";

export function startListening(nodeAddress: string) {
  console.log('Starting event handler...')
  const addr = nodeAddress + "/events/main"
  console.log(`Addr: ${addr}`)
  const es = new EventStream(nodeAddress + "/events/main");
  es.start()
  es.subscribe(EventName.DeployProcessed, async (event) => {
    console.log(`GOT EVENT ${event.id}`)
    console.log(event)
  })
}