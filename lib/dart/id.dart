import 'package:http/http.dart' as http;

void main() async {
  final didUrl = 'did:example:123456789abcdefghi';
  final resolverUrl = 'https://example-did-resolver.com/resolve';

  final response = await http.get('$resolverUrl?did=$didUrl');
  if (response.statusCode == 200) {
    final didDocument = response.body;
    print('Resolved DID Document: $didDocument');
  } else {
    print('Failed to resolve DID. Status code: ${response.statusCode}');
  }
}

