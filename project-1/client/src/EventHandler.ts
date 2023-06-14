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

import { Parser } from "@make-software/ces-js-parser";

function normalizeHash(contractHash: string): string {
  return contractHash.startsWith("hash-") ? contractHash.slice(5) : contractHash
}

export async function startListening(nodeEventsUrl: string, nodeRpcUrl: string, contractHash: string) {

  const casperClient = new CasperServiceByJsonRPC(nodeRpcUrl)
  const rpcClient = new CasperServiceByJsonRPC(nodeRpcUrl)

  const hashsesTowatch = [contractHash].map(normalizeHash)
  
  const parser = await Parser.create(rpcClient, hashsesTowatch)
  
  console.log('Starting event handler...')
  const es = new EventStream(nodeEventsUrl);
  es.start()
  es.subscribe(EventName.DeployProcessed, async (event) => {
    const executionResult = event.body.DeployProcessed.execution_result
    const events = await parser.parseExecutionResult(executionResult);
    if (events.length > 0) {
      console.log("----- Contract Events -----")
      events.forEach(event => {
        console.log("---- EV ----")
        console.log(event)
        console.log("---- EV-end ----")
      });
    }
  })
}