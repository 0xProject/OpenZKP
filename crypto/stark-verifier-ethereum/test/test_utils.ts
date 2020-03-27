/**
 * Turns a tx object from ethers into the events when it is executed
 * @param tx The tx to turn into events
 */
export async function txToEventsAsync(tx: any): Promise<any> {
    return (await (await tx).wait()).events;
}
