import socket
import argparse

def scan_port(host, port, protocol):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM if protocol == 'tcp' else socket.SOCK_DGRAM)
    sock.settimeout(1)

    try:
        sock.connect((host, port))
        if protocol == 'tcp':
            service = socket.getservbyport(port)
            print(f"Port {port} is open - Service: {service}")
        else:
            print(f"Port {port} is open")
    except socket.error:
        print(f"Port {port} is closed")
    finally:
        sock.close()

def main():
    parser = argparse.ArgumentParser(description="Simple Port Scanner")
    parser.add_argument('host', help='Target host IP address')
    parser.add_argument('-p', '--ports', type=str, help='Range of ports to scan (e.g., 80, 80-100)')
    parser.add_argument('-u', '--udp', action='store_true', help='Enable UDP scan')
    parser.add_argument('-t', '--tcp', action='store_true', help='Enable TCP scan')

    args = parser.parse_args()

    if not args.udp and not args.tcp:
        print("Please specify either TCP (-t) or UDP (-u) scan.")
        return

    if args.ports:
        port_ranges = args.ports.split(',')
        for port_range in port_ranges:
            if '-' in port_range:
                start, end = map(int, port_range.split('-'))
                for port in range(start, end + 1):
                    if args.tcp:
                        scan_port(args.host, port, 'tcp')
                    if args.udp:
                        scan_port(args.host, port, 'udp')
            else:
                port = int(port_range)
                if args.tcp:
                    scan_port(args.host, port, 'tcp')
                if args.udp:
                    scan_port(args.host, port, 'udp')

if __name__ == "__main__":
    main()
