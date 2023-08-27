import { Context, PersistentMap, u128, ContractPromiseBatch, PersistentDeque, context } from 'near-sdk-as'
import { FrameDataWrapper, FrameMetadata, HistoryEntry } from "./types";
import { toYocto, _randomNum } from './utils';

/**************************/
/* DATA TYPES AND VARIABLES */
/**************************/
type TokenId = u16

/**************************/
/* STORAGE & COLLECTIONS */
/**************************/

const frameDataMap = new PersistentMap<TokenId, String>("frameData");
const frameMetadataMap = new PersistentMap<TokenId, FrameMetadata>('frameMetadata');
const history = new PersistentDeque<HistoryEntry>("h");

/**************************/
/* READ METHODS */
/**************************/

export function load_frame_data(start: u16, end: u16): String[] {
  const output: String[] = [];

  for (let index = start; index <= end; index++) {
    output.push(frameDataMap.getSome(index));
  }

  return output;
}

export function load_frames(start: u16, end: u16): FrameDataWrapper {
  const output: FrameDataWrapper = { metadata: [], data: [] };

  for (let index = start; index < end; index++) {
    output.data.push(frameDataMap.getSome(index));
    output.metadata.push(frameMetadataMap.getSome(index));
  }

  return output;
}

/******************/
/* WRITE METHODS */
/******************/

export function resetImage(frameId: u16): void {
  _onlyContractOwner();
  _isValidFrameId(frameId);

  let initFrameData = "WzIzNCzfBN8E3wTfBN8E3wTfBN8E1AQxNjDJBN8s3wTOTN9U3wTWTN9U30zeBN9EzUzcVMwc30zRNNBY1GjfUNkE2CDfUN9Q31DfUN9Q31D/AUDXVNxQ2EjYVN9M/wD4/wCY3wTfTN8E31TfTN8E2VTfLN8E3wTfBN8E3wTJBF0=";
  frameDataMap.set(frameId, initFrameData);

  const framemeta = _getFramemetadata(frameId);
  framemeta.message = "Please avoid controversial content";
  frameMetadataMap.set(frameId, framemeta);
}

export function offerFrame(frameId: u16, price: u32): void {
  _isValidFrameId(frameId);
  _isFrameowner(frameId);
  assert(u128.from(price) >= u128.from(4), "Minimum price of a frame is 4 NEAR");
  const framemeta = _getFramemetadata(frameId);
  framemeta.price = price;
  frameMetadataMap.set(frameId, framemeta);
}

export function cancelOffer(frameId: u16): void {
  _isValidFrameId(frameId);
  _isFrameowner(frameId);
  const framemeta = _getFramemetadata(frameId);
  framemeta.price = 0;
  frameMetadataMap.set(frameId, framemeta);
}

export function buyFrame(frameId: u16): void {
  _isValidFrameId(frameId);
  let frameMetadata = frameMetadataMap.getSome(frameId);
  assert(frameMetadata.price >= 3, "Frame not for sale");
  assert(Context.attachedDeposit >= toYocto(frameMetadata.price), "Deposit too low");
  let history_entry = new HistoryEntry(frameId, Context.predecessor, frameMetadata.owner, frameMetadata.price.toString());
  frameMetadataMap.set(frameId, new FrameMetadata(0, Context.predecessor, "", "", context.blockTimestamp));
  _sendNear(frameMetadata.owner, frameMetadata.price);
  history.pushFront(history_entry);
  if (history.length > 20) {
    history.popBack();
  }
  //send pixeltoken
}

export function editFrameimage(frameId: u16, frameData: string): void {
  _isValidFrameId(frameId);
  _isCoauthor(frameId);
  frameDataMap.set(frameId, frameData);
}

export function editFrame(frameId: u16, frameData: string, message: string, coauthor: string): void {
  _isValidFrameId(frameId);
  _isFrameowner(frameId);
  assert(message.length <= 80, "Message max size of 80 characters exceeded.");
  assert(coauthor.length <= 100, "Coauthor max size of 100 characters exceeded.");
  assert(coauthor != context.predecessor, "Don't add yourself as Coauthor :P");
  frameDataMap.set(frameId, frameData);
  const framemeta = _getFramemetadata(frameId);
  framemeta.message = message;
  framemeta.coauthor = coauthor;
  frameMetadataMap.set(frameId, framemeta);
}

/****************/
/* VIEW METHODS */
/****************/

export function getHistory(): HistoryEntry[] | null {

  const historyResponse: HistoryEntry[] = [];
  for (let i = 0; i < Math.min(history.length, 20); i++) {
    historyResponse.push(history[i]);
  }
  return historyResponse;
}

/****************/
/* PRIVATE METHODS */
/****************/

function _sendNear(recipient: string, amount: u32): void {
  if (recipient == context.contractName) {
    //ContractPromiseBatch.create(secondWallet).transfer(_convertNear("0.5"));
  }
  else {
    const near_amount_yocto = toYocto(amount - u32(1));
    ContractPromiseBatch.create(recipient).transfer(near_amount_yocto);
    //ContractPromiseBatch.create(secondWallet).transfer(_convertNear("0.5"));
  }
}

function _isValidFrameId(frameId: u16): void {
  assert(0 <= frameId && frameId < 600, "FrameId not valid");
}

function _isFrameowner(frameId: u16): void {
  let frameMetaData = _getFramemetadata(frameId);
  assert(frameMetaData.owner == context.predecessor || frameMetaData.owner == context.sender, "This frame does not belong to you");
}

function _isCoauthor(frameId: u16): void {
  let frameMetaData = _getFramemetadata(frameId);
  assert(frameMetaData.coauthor == context.predecessor, "You are not the Coauthor of this frame");
}

function _onlyContractOwner(): void {
  assert(context.contractName == Context.predecessor, "only the contractowner can call this function");
}

function _getFramemetadata(frameId: u16): FrameMetadata {
  return frameMetadataMap.getSome(frameId);
}
