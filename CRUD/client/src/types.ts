import { Contract } from "near-api-js";

export interface CURDContract extends Contract {
  create_update: (key: string, value: string) => Promise<void>;
  read: ({ k }: { k: string }) => Promise<string>;
  delete: (key: string) => Promise<void>;
  read_keys: () => Promise<string[]>;
}
