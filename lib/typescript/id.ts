import { Resolver } from 'did-resolver';
import fetch from 'node-fetch';

async function resolveDid(didUrl: string, resolverUrl: string) {
  const resolver = new Resolver({ fetch });
  try {
    const didDocument = await resolver.resolve(didUrl);
    console.log('Resolved DID Document:', didDocument);
  } catch (error) {
    console.error('Failed to resolve DID:', error);
  }
}

const didUrl = 'did:example:123456789abcdefghi';
const resolverUrl = 'https://example-did-resolver.com/resolve';
resolveDid(didUrl, resolverUrl);

