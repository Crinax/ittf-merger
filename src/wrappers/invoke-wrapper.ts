import { invoke as _invoke } from '@tauri-apps/api';

type EmptyArguments = void;

export type Message = {
  'GENERATE_EXCEL_FILE': [EmptyArguments, {}],
};

export const message: Record<keyof Message, keyof Message> = {
  GENERATE_EXCEL_FILE: 'GENERATE_EXCEL_FILE'
}

export const invoke = <
  T extends keyof Message,
  Args extends Message[T][0],
  Result extends Message[T][1]
>(message: T, args?: Args): Promise<Result> => 
  args
    ? _invoke(message.toLowerCase(), args)
    : _invoke(message.toLowerCase());
