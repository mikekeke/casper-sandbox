import {
  EventStream,
  EventName,
  CasperServiceByJsonRPC
} from "casper-js-sdk";

import { Event, Parser, fetchContractSchemasBytes, parseSchemasFromBytes, parseEventNameAndData } from "@make-software/ces-js-parser";
import { ExampleContractClient } from "./ExampleContractClient";

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



export class EventHandler {
  constructor(readonly ecClient: ExampleContractClient) { }

  public async startListening(processEvent: (event: SomeEvent| undefined) => void) {
    const rpcClient = new CasperServiceByJsonRPC(this.ecClient.nodeRpcUrl)

    const normalizedHash = normalizeHash(this.ecClient.getContractHash())

    const parser = await Parser.create(rpcClient, [normalizedHash])

    const es = new EventStream(this.ecClient.nodeEventsUrl);
    es.start()
    es.subscribe(EventName.DeployProcessed, async (event) => {
      const executionResult = event.body.DeployProcessed.execution_result
      const parseResults = parser.parseExecutionResult(executionResult);
      if (parseResults.length > 0) {
        parseResults.map(pr => SomeEvent.fromEvent(pr.event)).forEach(processEvent);
      }
    })
  }
}

function normalizeHash(contractHash: string): string {
  return contractHash.startsWith("hash-") ? contractHash.slice(5) : contractHash
}
