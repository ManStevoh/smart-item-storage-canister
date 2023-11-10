import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Error = { 'NotFound' : { 'msg' : string } };
export interface SmartStorageItem {
  'id' : bigint,
  'updated_at' : [] | [bigint],
  'name' : string,
  'description' : string,
  'created_at' : bigint,
  'is_available' : boolean,
  'location' : string,
}
export interface SmartStorageItemPayload {
  'name' : string,
  'description' : string,
  'is_available' : boolean,
  'location' : string,
}
export interface _SERVICE {
  'add_smart_storage_item' : ActorMethod<
    [SmartStorageItemPayload],
    [] | [SmartStorageItem]
  >,
  'delete_smart_storage_item' : ActorMethod<
    [bigint],
    { 'Ok' : SmartStorageItem } |
      { 'Err' : Error }
  >,
  'get_all_smart_storage_items' : ActorMethod<[], Array<SmartStorageItem>>,
  'get_available_smart_storage_items' : ActorMethod<
    [],
    Array<SmartStorageItem>
  >,
  'get_smart_storage_item' : ActorMethod<
    [bigint],
    { 'Ok' : SmartStorageItem } |
      { 'Err' : Error }
  >,
  'is_item_available' : ActorMethod<
    [bigint],
    { 'Ok' : boolean } |
      { 'Err' : Error }
  >,
  'mark_item_as_available' : ActorMethod<
    [bigint],
    { 'Ok' : SmartStorageItem } |
      { 'Err' : Error }
  >,
  'mark_item_as_unavailable' : ActorMethod<
    [bigint],
    { 'Ok' : SmartStorageItem } |
      { 'Err' : Error }
  >,
  'search_smart_storage_items' : ActorMethod<[string], Array<SmartStorageItem>>,
  'update_smart_storage_item' : ActorMethod<
    [bigint, SmartStorageItemPayload],
    { 'Ok' : SmartStorageItem } |
      { 'Err' : Error }
  >,
}
