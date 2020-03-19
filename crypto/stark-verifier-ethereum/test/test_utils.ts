export async function tx_to_events(tx: any): Promise<any> {
  return (await (await tx).wait()).events;
}
