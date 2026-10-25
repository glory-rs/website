"""Static preview with Cloudflare Pages' nearest-ancestor 404 behavior."""
import argparse
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import urlsplit


class PagesHandler(SimpleHTTPRequestHandler):
    def send_error(self, code, message=None, explain=None):
        if code == 404:
            path = urlsplit(self.path).path
            # The published site has one root and one Chinese 404 document.
            name = "zh/404.html" if path == "/zh" or path.startswith("/zh/") else "404.html"
            page = Path(self.directory, name)
            if page.is_file():
                body = page.read_bytes()
                self.send_response(404)
                self.send_header("Content-Type", "text/html; charset=utf-8")
                self.send_header("Content-Length", str(len(body)))
                self.end_headers()
                if self.command != "HEAD":
                    self.wfile.write(body)
                return
        super().send_error(code, message, explain)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--directory", required=True)
    parser.add_argument("--port", type=int, default=8796)
    options = parser.parse_args()
    handler = partial(PagesHandler, directory=options.directory)
    print(f"Serving {options.directory} on http://127.0.0.1:{options.port}", flush=True)
    ThreadingHTTPServer(("127.0.0.1", options.port), handler).serve_forever()
