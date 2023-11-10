export const idlFactory = ({ IDL }) => {
  const SmartStorageItemPayload = IDL.Record({
    'name' : IDL.Text,
    'description' : IDL.Text,
    'is_available' : IDL.Bool,
    'location' : IDL.Text,
  });
  const SmartStorageItem = IDL.Record({
    'id' : IDL.Nat64,
    'updated_at' : IDL.Opt(IDL.Nat64),
    'name' : IDL.Text,
    'description' : IDL.Text,
    'created_at' : IDL.Nat64,
    'is_available' : IDL.Bool,
    'location' : IDL.Text,
  });
  const Error = IDL.Variant({ 'NotFound' : IDL.Record({ 'msg' : IDL.Text }) });
  return IDL.Service({
    'add_smart_storage_item' : IDL.Func(
        [SmartStorageItemPayload],
        [IDL.Opt(SmartStorageItem)],
        [],
      ),
    'delete_smart_storage_item' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : SmartStorageItem, 'Err' : Error })],
        [],
      ),
    'get_all_smart_storage_items' : IDL.Func(
        [],
        [IDL.Vec(SmartStorageItem)],
        [],
      ),
    'get_available_smart_storage_items' : IDL.Func(
        [],
        [IDL.Vec(SmartStorageItem)],
        [],
      ),
    'get_smart_storage_item' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : SmartStorageItem, 'Err' : Error })],
        ['query'],
      ),
    'is_item_available' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : Error })],
        ['query'],
      ),
    'mark_item_as_available' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : SmartStorageItem, 'Err' : Error })],
        [],
      ),
    'mark_item_as_unavailable' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : SmartStorageItem, 'Err' : Error })],
        [],
      ),
    'search_smart_storage_items' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(SmartStorageItem)],
        [],
      ),
    'update_smart_storage_item' : IDL.Func(
        [IDL.Nat64, SmartStorageItemPayload],
        [IDL.Variant({ 'Ok' : SmartStorageItem, 'Err' : Error })],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
