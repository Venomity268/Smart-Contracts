import { u128 } from "near-sdk-as";

@nearBindgen
export class FrameMetadata {
  constructor(
    public price: u32,
    public owner: string,
    public message: string,
    public coauthor: string,
    public ts: u64
  ) {
  }
}

@nearBindgen
export class HistoryEntry {
  constructor(public frameId: u16, public buyer: string, public seller: string, public price: string) {
  }
}


@nearBindgen
export class AddEggMessage {
  public account_id: string;
  public rarity: u32;
}

@nearBindgen
export class FrameDataWrapper {
  public metadata: FrameMetadata[];
  public data: String[];
}