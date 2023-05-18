export interface Firmware {
  path: string;
  address: string;
  filename?: string;
  description?: string;
}

export interface FirmwareCard  {
  port: string;
  check: boolean;
  firmwareList:Firmware[],
  progress: {
    value: number;
    visible: boolean;
  };
}
