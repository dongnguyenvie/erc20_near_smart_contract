import { Contract } from "near-api-js";

export interface CURDContract extends Contract {
  create_update: ({ k, v }: { k: string; v: string }) => Promise<void>;
  read: ({ k }: { k: string }) => Promise<string>;
  delete: ({ k }: { k: string }) => Promise<void>;
  read_keys: () => Promise<string[]>;
}
