# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 15:33:16
- **数据来源**: connectivity_results-20251212-153107.json
- **总测试数**: 355
- **失败测试数**: 16
- **成功测试数**: 339
- **失败率**: 4.51%
- **平均延迟**: 1127.53ms
- **最小延迟**: 187ms
- **最大延迟**: 9208ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 15:33:16
- **IP地址**: 2408:840d:8620:427:799f:10ac:b5a7:6c8f
- **国家/地区**: China (CN)
- **ASN**: AS17621
- **网络组织**: China Unicom Shanghai network
- **网络域名**: chinaunicom.cn
- **大洲**: Asia (AS)
- **地理坐标**: 34.7732, 113.722
- **时区**: Asia/Shanghai
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: 上下文超时**: 8 次 (50.0%)
- **连接超时: I/O超时**: 5 次 (31.3%)
- **DNS解析错误: 其他DNS错误**: 3 次 (18.8%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: 上下文超时 (8 次测试)

| 序号 | 主机/域名                | 目标IP                    | IP版本  | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                              |
| ---- | ------------------------ | ------------------------- | ------- | ---- | ------ | -------- | ------ | ----------------------------------------------------------------------------------------------------- |
| 32   | cmcc.877774.xyz          | Unknown                   | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 106  | comicabc.com             | 104.21.64.10              | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 153  | icook.hk                 | 104.21.90.210             | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 155  | icook.hk                 | 2606:4700:3037::ac43:a168 | IPv6    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 177  | braden.ns.cloudflare.com | Unknown                   | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 256  | uriah.ns.cloudflare.com  | Unknown                   | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 337  | yx-auto.pages.dev        | 172.67.161.98             | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 351  | www.7749tv.com           | Unknown                   | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |

#### 连接超时: I/O超时 (5 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 42   | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 74   | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 217  | 141.147.185.63   | 141.147.185.63 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 141.147.185.63:443: i/o timeout |
| 303  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 346  | 3.0.50.69        | 3.0.50.69      | IPv4   | none | N/A    | 0        | N/A    | dial tcp 3.0.50.69:443: i/o timeout      |

#### DNS解析错误: 其他DNS错误 (3 次测试)

| 序号 | 主机/域名        | 目标IP  | IP版本  | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息      |
| ---- | ---------------- | ------- | ------- | ---- | ------ | -------- | ------ | ------------- |
| 75   | www.okcupid.com  | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析无结果 |
| 251  | www.visa.com.hk  | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析无结果 |
| 317  | cfip.1323123.xyz | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析无结果 |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 13 次 (81.3%)
- **DNS解析错误**: 3 次 (18.8%)

#### 错误模式分析

**超时集中度分析**: 共有 5 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 12 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 8 次，IPv6失败 1 次，两种协议都存在问题

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                   | 目标IP                               | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | --------------------------- | ------------------------------------ | ------ | ---- | ------- | -------- | ---------- |
| 213  | ip.sb                       | 172.67.75.172                        | IPv4   | h3   | ✅ 成功 | 187      | cloudflare |
| 220  | singapore.com               | 172.67.75.194                        | IPv4   | h3   | ✅ 成功 | 189      | cloudflare |
| 184  | www.ipchicken.com           | 172.67.68.101                        | IPv4   | h3   | ✅ 成功 | 193      | cloudflare |
| 219  | singapore.com               | 104.26.12.140                        | IPv4   | h3   | ✅ 成功 | 193      | cloudflare |
| 225  | 456.cloudflare.182682.xyz   | 104.26.9.160                         | IPv4   | h3   | ✅ 成功 | 193      | cloudflare |
| 318  | japan.com                   | 104.26.4.60                          | IPv4   | h3   | ✅ 成功 | 196      | cloudflare |
| 320  | japan.com                   | 172.67.70.92                         | IPv4   | h3   | ✅ 成功 | 197      | cloudflare |
| 147  | www.whatismyip.com          | 172.67.69.129                        | IPv4   | h3   | ✅ 成功 | 198      | cloudflare |
| 68   | cu.877774.xyz               | 104.26.4.116                         | IPv4   | h3   | ✅ 成功 | 200      | cloudflare |
| 58   | iplocation.io               | 104.26.10.222                        | IPv4   | h3   | ✅ 成功 | 202      | cloudflare |
| 69   | cu.877774.xyz               | 104.26.4.117                         | IPv4   | h3   | ✅ 成功 | 204      | cloudflare |
| 71   | www.hugedomains.com         | 172.67.70.191                        | IPv4   | h3   | ✅ 成功 | 204      | cloudflare |
| 224  | 456.cloudflare.182682.xyz   | 104.26.8.160                         | IPv4   | h3   | ✅ 成功 | 204      | cloudflare |
| 26   | 172.67.75.172               | 172.67.75.172                        | IPv4   | h3   | ✅ 成功 | 206      | cloudflare |
| 319  | japan.com                   | 104.26.5.60                          | IPv4   | h3   | ✅ 成功 | 206      | cloudflare |
| 56   | iplocation.io               | 172.67.70.100                        | IPv4   | h3   | ✅ 成功 | 211      | cloudflare |
| 67   | cu.877774.xyz               | 104.26.4.115                         | IPv4   | h3   | ✅ 成功 | 214      | cloudflare |
| 116  | toy-people.com              | 104.26.2.36                          | IPv4   | h3   | ✅ 成功 | 214      | cloudflare |
| 65   | cu.877774.xyz               | 104.26.4.113                         | IPv4   | h3   | ✅ 成功 | 215      | cloudflare |
| 183  | www.ipchicken.com           | 104.26.7.112                         | IPv4   | h3   | ✅ 成功 | 216      | cloudflare |
| 294  | 104.26.13.31                | 104.26.13.31                         | IPv4   | h3   | ✅ 成功 | 217      | cloudflare |
| 148  | www.whatismyip.com          | 104.26.13.23                         | IPv4   | h3   | ✅ 成功 | 218      | cloudflare |
| 212  | ip.sb                       | 104.26.13.31                         | IPv4   | h3   | ✅ 成功 | 218      | cloudflare |
| 10   | ipv4.ip.sb                  | 172.67.75.172                        | IPv4   | h3   | ✅ 成功 | 220      | cloudflare |
| 73   | www.hugedomains.com         | 104.26.6.37                          | IPv4   | h3   | ✅ 成功 | 223      | cloudflare |
| 62   | cu.877774.xyz               | 104.26.4.119                         | IPv4   | h3   | ✅ 成功 | 226      | cloudflare |
| 277  | 172.67.79.211               | 172.67.79.211                        | IPv4   | h3   | ✅ 成功 | 227      | cloudflare |
| 70   | cu.877774.xyz               | 104.26.4.118                         | IPv4   | h3   | ✅ 成功 | 228      | cloudflare |
| 66   | cu.877774.xyz               | 104.26.4.114                         | IPv4   | h3   | ✅ 成功 | 229      | cloudflare |
| 211  | ip.sb                       | 104.26.12.31                         | IPv4   | h3   | ✅ 成功 | 230      | cloudflare |
| 117  | toy-people.com              | 172.67.72.18                         | IPv4   | h3   | ✅ 成功 | 234      | cloudflare |
| 218  | singapore.com               | 104.26.13.140                        | IPv4   | h3   | ✅ 成功 | 236      | cloudflare |
| 115  | toy-people.com              | 104.26.3.36                          | IPv4   | h3   | ✅ 成功 | 265      | cloudflare |
| 146  | www.whatismyip.com          | 104.26.12.23                         | IPv4   | h3   | ✅ 成功 | 267      | cloudflare |
| 31   | 104.26.6.112                | 104.26.6.112                         | IPv4   | h3   | ✅ 成功 | 280      | cloudflare |
| 41   | 168.138.165.174             | 168.138.165.174                      | IPv4   | h2   | ✅ 成功 | 280      | cloudflare |
| 11   | ipv4.ip.sb                  | 104.26.12.31                         | IPv4   | h3   | ✅ 成功 | 294      | cloudflare |
| 57   | iplocation.io               | 104.26.11.222                        | IPv4   | h3   | ✅ 成功 | 312      | cloudflare |
| 226  | 456.cloudflare.182682.xyz   | 172.67.75.208                        | IPv4   | h3   | ✅ 成功 | 357      | cloudflare |
| 72   | www.hugedomains.com         | 104.26.7.37                          | IPv4   | h3   | ✅ 成功 | 363      | cloudflare |
| 209  | www.glassdoor.com           | 104.17.64.70                         | IPv4   | h3   | ✅ 成功 | 367      | cloudflare |
| 20   | 104.16.45.84                | 104.16.45.84                         | IPv4   | h3   | ✅ 成功 | 379      | cloudflare |
| 274  | www.udemy.com               | 104.16.142.237                       | IPv4   | h3   | ✅ 成功 | 401      | cloudflare |
| 46   | cfip.xxxxxxxx.tk            | 190.93.244.201                       | IPv4   | h3   | ✅ 成功 | 406      | cloudflare |
| 63   | cu.877774.xyz               | 104.26.4.111                         | IPv4   | h3   | ✅ 成功 | 406      | cloudflare |
| 281  | www.digitalocean.com        | 104.19.174.68                        | IPv4   | h3   | ✅ 成功 | 413      | cloudflare |
| 273  | www.udemy.com               | 104.16.143.237                       | IPv4   | h3   | ✅ 成功 | 414      | cloudflare |
| 279  | 104.17.142.12               | 104.17.142.12                        | IPv4   | h3   | ✅ 成功 | 415      | cloudflare |
| 292  | 104.18.78.214               | 104.18.78.214                        | IPv4   | h3   | ✅ 成功 | 416      | cloudflare |
| 157  | asia.877774.xyz             | 104.17.142.146                       | IPv4   | h3   | ✅ 成功 | 421      | cloudflare |
| 163  | 104.17.68.85                | 104.17.68.85                         | IPv4   | h3   | ✅ 成功 | 426      | cloudflare |
| 95   | pranab.ns.cloudflare.com    | 2a06:98c1:50::ac40:23c7              | IPv6   | h3   | ✅ 成功 | 432      | cloudflare |
| 140  | dylan.ns.cloudflare.com     | 162.159.44.187                       | IPv4   | h3   | ✅ 成功 | 438      | cloudflare |
| 196  | moura.ns.cloudflare.com     | 2a06:98c1:50::ac40:23d9              | IPv6   | h3   | ✅ 成功 | 438      | cloudflare |
| 158  | asia.877774.xyz             | 104.16.211.153                       | IPv4   | h3   | ✅ 成功 | 439      | cloudflare |
| 237  | ashton.ns.cloudflare.com    | 2606:4700:58::a29f:2cad              | IPv6   | h3   | ✅ 成功 | 439      | cloudflare |
| 310  | lewis.ns.cloudflare.com     | 172.64.35.159                        | IPv4   | h3   | ✅ 成功 | 447      | cloudflare |
| 335  | www.wto.org                 | 2606:4700:4406::ac40:9242            | IPv6   | h3   | ✅ 成功 | 447      | cloudflare |
| 49   | www.4chan.org               | 104.16.228.229                       | IPv4   | h3   | ✅ 成功 | 448      | cloudflare |
| 182  | www.ipchicken.com           | 104.26.6.112                         | IPv4   | h3   | ✅ 成功 | 449      | cloudflare |
| 134  | 104.17.79.11                | 104.17.79.11                         | IPv4   | h3   | ✅ 成功 | 450      | cloudflare |
| 166  | cf.zhetengsha.eu.org        | 2606:4700:440a::ac40:98f1            | IPv6   | h3   | ✅ 成功 | 451      | cloudflare |
| 161  | bestcf.030101.xyz           | 2606:4700:0:81e7:c232:ce8d:abff:90e0 | IPv6   | h3   | ✅ 成功 | 452      | cloudflare |
| 270  | trevor.ns.cloudflare.com    | 2606:4700:58::a29f:2c9a              | IPv6   | h3   | ✅ 成功 | 456      | cloudflare |
| 165  | cf.zhetengsha.eu.org        | 2a06:98c1:3105::6812:230f            | IPv6   | h3   | ✅ 成功 | 457      | cloudflare |
| 205  | rustam.ns.cloudflare.com    | 2a06:98c1:50::ac40:2394              | IPv6   | h3   | ✅ 成功 | 457      | cloudflare |
| 238  | ashton.ns.cloudflare.com    | 2803:f800:50::6ca2:c3ad              | IPv6   | h3   | ✅ 成功 | 459      | cloudflare |
| 6    | ct.877774.xyz               | 172.64.229.173                       | IPv4   | h3   | ✅ 成功 | 460      | cloudflare |
| 126  | decker.ns.cloudflare.com    | 2a06:98c1:50::ac40:239b              | IPv6   | h3   | ✅ 成功 | 460      | cloudflare |
| 159  | bestcf.030101.xyz           | 104.19.148.231                       | IPv4   | h3   | ✅ 成功 | 460      | cloudflare |
| 9    | ipv4.ip.sb                  | 104.26.13.31                         | IPv4   | h3   | ✅ 成功 | 461      | cloudflare |
| 246  | julio.ns.cloudflare.com     | 172.64.35.209                        | IPv4   | h3   | ✅ 成功 | 463      | cloudflare |
| 314  | otto.ns.cloudflare.com      | 108.162.195.135                      | IPv4   | h3   | ✅ 成功 | 463      | cloudflare |
| 82   | craig.ns.cloudflare.com     | 108.162.195.192                      | IPv4   | h3   | ✅ 成功 | 464      | cloudflare |
| 296  | damien.ns.cloudflare.com    | 162.159.44.168                       | IPv4   | h3   | ✅ 成功 | 464      | cloudflare |
| 306  | icook.tw                    | 2606:4700:10::ac42:9e73              | IPv6   | h3   | ✅ 成功 | 464      | cloudflare |
| 8    | ct.877774.xyz               | 172.64.229.185                       | IPv4   | h3   | ✅ 成功 | 465      | cloudflare |
| 48   | www.4chan.org               | 104.16.229.229                       | IPv4   | h3   | ✅ 成功 | 467      | cloudflare |
| 133  | 104.16.223.179              | 104.16.223.179                       | IPv4   | h3   | ✅ 成功 | 467      | cloudflare |
| 207  | 162.159.36.104              | 162.159.36.104                       | IPv4   | h3   | ✅ 成功 | 467      | cloudflare |
| 124  | decker.ns.cloudflare.com    | 2606:4700:58::a29f:2c9b              | IPv6   | h3   | ✅ 成功 | 468      | cloudflare |
| 276  | tasteatlas.com              | 2606:4700::6811:2569                 | IPv6   | h3   | ✅ 成功 | 468      | cloudflare |
| 35   | 103.160.204.59              | 103.160.204.59                       | IPv4   | h3   | ✅ 成功 | 470      | cloudflare |
| 197  | benedict.ns.cloudflare.com  | 108.162.195.205                      | IPv4   | h3   | ✅ 成功 | 470      | cloudflare |
| 315  | otto.ns.cloudflare.com      | 162.159.44.135                       | IPv4   | h3   | ✅ 成功 | 470      | cloudflare |
| 248  | whatismyipaddress.com       | 2606:4700::6813:df4f                 | IPv6   | h3   | ✅ 成功 | 472      | cloudflare |
| 245  | julio.ns.cloudflare.com     | 162.159.44.209                       | IPv4   | h3   | ✅ 成功 | 473      | cloudflare |
| 249  | whatismyipaddress.com       | 2606:4700::6813:de4f                 | IPv6   | h3   | ✅ 成功 | 473      | cloudflare |
| 332  | www.wto.org                 | 172.64.146.66                        | IPv4   | h3   | ✅ 成功 | 473      | cloudflare |
| 15   | www.pcmag.com               | 2606:4700::6810:1476                 | IPv6   | h3   | ✅ 成功 | 474      | cloudflare |
| 235  | ashton.ns.cloudflare.com    | 162.159.44.173                       | IPv4   | h3   | ✅ 成功 | 474      | cloudflare |
| 278  | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304            | IPv6   | h3   | ✅ 成功 | 476      | cloudflare |
| 243  | yx-auto.pages.dev           | 2606:4700:310c::ac42:2f70            | IPv6   | h3   | ✅ 成功 | 477      | cloudflare |
| 255  | 104.18.14.76                | 104.18.14.76                         | IPv4   | h3   | ✅ 成功 | 477      | cloudflare |
| 199  | benedict.ns.cloudflare.com  | 172.64.35.205                        | IPv4   | h3   | ✅ 成功 | 478      | cloudflare |
| 242  | yx-auto.pages.dev           | 2606:4700:310c::ac42:2c90            | IPv6   | h3   | ✅ 成功 | 478      | cloudflare |
| 200  | benedict.ns.cloudflare.com  | 2606:4700:58::a29f:2ccd              | IPv6   | h3   | ✅ 成功 | 479      | cloudflare |
| 282  | www.digitalocean.com        | 2606:4700::6813:ad44                 | IPv6   | h3   | ✅ 成功 | 479      | cloudflare |
| 51   | huxley.ns.cloudflare.com    | 162.159.44.188                       | IPv4   | h3   | ✅ 成功 | 480      | cloudflare |
| 94   | pranab.ns.cloudflare.com    | 2803:f800:50::6ca2:c3c7              | IPv6   | h3   | ✅ 成功 | 480      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 8 条记录
- **慢 (200-500ms)**: 92 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 8 次
- **IPv6 失败**: 1 次

### 按协议统计

- **none**: 12 次失败
- **h2**: 4 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
