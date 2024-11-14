#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct CConfig {
  bool listen_route;
  char *config_name;
  unsigned int node_ipv4;
  uint8_t *node_ipv6;
  unsigned char prefix;
  unsigned char prefix_v6;
  char *tun_name;
  char *encrypt;
  char *algorithm;
  unsigned short port;
  char *group_code;
  char **peer;
  unsigned int peer_count;
  char *bind_dev_name;
  unsigned int exit_node;
  char **udp_stun;
  unsigned int udp_stun_count;
  char **tcp_stun;
  unsigned int tcp_stun_count;
};

struct CRouteItem {
  unsigned int node_id;
  char *next_hop;
  char *protocol;
  unsigned char metric;
  unsigned int rtt;
  char *interface;
};

struct CRouteItemVec {
  CRouteItem *ptr;
  unsigned int length;
};

struct CGroupItem {
  char *group_code;
  unsigned int node_num;
};

struct CGroupItemVec {
  CGroupItem *ptr;
  unsigned int length;
};

struct CNetworkNatInfo {
  unsigned int node_ip;
  unsigned int local_ipv4;
  char *ipv6;
  char *nat_type;
  unsigned int *public_ips;
  unsigned int public_ips_len;
  unsigned short *public_udp_ports;
  unsigned int public_udp_ports_len;
  unsigned short public_tcp_port;
  unsigned short *local_udp_ports;
  unsigned int local_udp_ports_len;
  unsigned short local_tcp_port;
};

extern "C" {

void initialize_async_runtime();

NetLinkCoreApi *create_netlink_api(const CConfig *config);

NetLinkCoreApi *create_netlink_api_with_fd(CConfig *config, unsigned int tun_fd);

void drop_netlink_api(NetLinkCoreApi *api);

CRouteItemVec *get_current_nodes(NetLinkCoreApi *api);

CRouteItemVec *get_nodes_by_group(NetLinkCoreApi *api, char *group_code);

CGroupItemVec *get_groups(NetLinkCoreApi *api);

CNetworkNatInfo *get_current_info(NetLinkCoreApi *api);

void drop_CRouteItemVec(CRouteItemVec *route_items);

void drop_CGroupItemVec(CGroupItemVec *list);

void drop_CNetworkNatInfo(CNetworkNatInfo *p);

}  // extern "C"
