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

import { Event, Parser, fetchContractSchemasBytes, parseSchemasFromBytes, parseEventNameAndData } from "@make-software/ces-js-parser";

class SomeEvent {
  readonly emittedBy: string

  private constructor(emitedBy: string) {
    this.emittedBy = emitedBy
  }

  public static fromEvent(event: Event): SomeEvent | undefined {
    if (event.name != "SomeEvent" || event.data["emitted_by"] == undefined) {
      return undefined
    }
    return new SomeEvent(event.data.emitted_by.data)
  }
}

function normalizeHash(contractHash: string): string {
  return contractHash.startsWith("hash-") ? contractHash.slice(5) : contractHash
}

export async function startListening(nodeEventsUrl: string, nodeRpcUrl: string, contractHash: string) {

  const casperClient = new CasperServiceByJsonRPC(nodeRpcUrl)
  const rpcClient = new CasperServiceByJsonRPC(nodeRpcUrl)

  const normalizedHash = normalizeHash(contractHash)

  const parser = await Parser.create(rpcClient, [normalizedHash])

  const rootHash = await casperClient.getStateRootHash()

  const schemaBytes = await fetchContractSchemasBytes(rpcClient, normalizedHash, rootHash)
  const schemas = parseSchemasFromBytes(schemaBytes);

  console.log(`Schema: ${JSON.stringify(schemas, null, 2)}`)

  console.log('Starting event handler...')
  const es = new EventStream(nodeEventsUrl);
  es.start()
  es.subscribe(EventName.DeployProcessed, async (event) => {
    const executionResult = event.body.DeployProcessed.execution_result
    const parseResults = await parser.parseExecutionResult(executionResult);
    if (parseResults.length > 0) {
      console.log("----- Contract Events -----")
      parseResults.map(pr => pr.event).forEach(event => {
        console.log("---- EV ----")
        console.log(SomeEvent.fromEvent(event))
      });
    }
  })
  
  es.stop()
}