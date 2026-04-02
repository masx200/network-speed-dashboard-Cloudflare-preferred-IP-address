# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 12:12:12
- **数据来源**: connectivity_results-20251212-121212.json
- **总测试数**: 445
- **失败测试数**: 2
- **成功测试数**: 443
- **失败率**: 0.45%
- **平均延迟**: 53.67ms
- **最小延迟**: 33ms
- **最大延迟**: 677ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 12:12:12
- **IP地址**: 2a09:bac1:7680:28::4cf:47
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 37.1835, -121.7714
- **时区**: America/Los_Angeles
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 2 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 38   | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 368  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP                    | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | ------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 98   | 456.cloudflare.182682.xyz             | 2606:4700:20::681a:9a0    | IPv6   | h2   | ✅ 成功 | 33       | cloudflare |
| 152  | ipv4.ip.sb                            | 104.26.12.31              | IPv4   | h2   | ✅ 成功 | 33       | cloudflare |
| 174  | www.gov.ua                            | 2606:4700:3031::6815:1748 | IPv6   | h2   | ✅ 成功 | 33       | cloudflare |
| 377  | icook.tw                              | 2606:4700:10::6814:1c4a   | IPv6   | h2   | ✅ 成功 | 33       | cloudflare |
| 7    | www.ipget.net                         | 2606:4700:3036::6815:fd4  | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 16   | time.is                               | 2606:4700:20::681a:c36    | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 73   | ip.sb                                 | 2606:4700:20::681a:c1f    | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 88   | singapore.com                         | 2606:4700:20::681a:d8c    | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 170  | 172.67.75.172                         | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 173  | www.gov.ua                            | 2606:4700:3033::ac43:d17f | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 182  | 104.26.6.112                          | 104.26.6.112              | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 242  | www.okcupid.com                       | 104.18.160.63             | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 246  | 4444.cloudflare.182682.xyz                         | 2606:4700:3033::ac43:98b7 | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 251  | zread.ai                              | 2606:4700:3032::ac43:ca4e | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 257  | www.hugedomains.com                   | 2606:4700:20::681a:725    | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 267  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90 | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 300  | toy-people.com                        | 2606:4700:20::ac43:4812   | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 317  | cloudflare-ip.mofashi.ltd             | 104.21.72.233             | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 375  | icook.tw                              | 104.20.28.74              | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 391  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47  | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 55   | ct.877774.xyz                         | 172.64.229.236            | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 72   | ip.sb                                 | 2606:4700:20::ac43:4bac   | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 83   | ip.gs                                 | 2606:4700:3036::6815:eb0  | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 86   | singapore.com                         | 172.67.75.194             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 89   | singapore.com                         | 2606:4700:20::ac43:4bc2   | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 95   | 456.cloudflare.182682.xyz             | 104.26.9.160              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 157  | cf.0sm.com                            | 2606:4700:3032::6815:785  | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 168  | steamdb.info                          | 2606:4700:10::6814:22d4   | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 190  | iplocation.io                         | 104.26.11.222             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 201  | icook.hk                              | 2606:4700:3037::ac43:a168 | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 224  | cmcc.877774.xyz                       | 104.16.149.7              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 238  | www.okcupid.com                       | 104.16.223.254            | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 252  | zread.ai                              | 2606:4700:3033::6815:4cf0 | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 271  | na.877774.xyz                         | 104.18.38.235             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 272  | cu.877774.xyz                         | 104.26.4.114              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 299  | toy-people.com                        | 172.67.72.18              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 301  | toy-people.com                        | 2606:4700:20::681a:324    | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 330  | www.whatismyip.com                    | 104.26.13.23              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 332  | www.whatismyip.com                    | 2606:4700:20::ac43:4581   | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 336  | asia.877774.xyz                       | 104.17.139.62             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 340  | bestcf.030101.xyz                     | 104.17.99.183             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 349  | 172.64.151.55                         | 172.64.151.55             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 372  | cf.090227.xyz                         | 2a06:98c1:3105::6812:230f | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 379  | 104.17.142.12                         | 104.17.142.12             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 434  | japan.com                             | 172.67.70.92              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 5    | www.ipget.net                         | 104.21.15.212             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 11   | comicabc.com                          | 2606:4700:3030::ac43:ae15 | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 13   | time.is                               | 104.26.12.54              | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 18   | time.is                               | 2606:4700:20::ac43:449d   | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 39   | cfip.xxxxxxxx.tk                      | 104.16.241.229            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 41   | cfip.xxxxxxxx.tk                      | 190.93.244.201            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 82   | ip.gs                                 | 172.67.160.28             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 96   | 456.cloudflare.182682.xyz             | 172.67.75.208             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 102  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 109  | 162.159.133.85                        | 162.159.133.85            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 118  | dnschecker.org                        | 104.26.6.89               | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 125  | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 139  | tasteatlas.com                        | 104.17.37.105             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 142  | tasteatlas.com                        | 2606:4700::6811:2569      | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 144  | www.udemy.com                         | 104.16.143.237            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 155  | cf.0sm.com                            | 172.67.187.145            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 162  | ipinfo.in                             | 2606:4700:3037::ac43:c6cb | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 187  | 104.18.254.88                         | 104.18.254.88             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 188  | cf.877774.xyz                         | cf.877774.xyz             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 230  | www.4444.cloudflare.182682.xyz                           | 162.159.153.2             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 245  | 4444.cloudflare.182682.xyz                         | 2606:4700:3033::6815:50b4 | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 247  | www.visa.com.sg                       | 104.18.13.229             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 253  | www.hugedomains.com                   | 104.26.6.37               | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 268  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70 | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 273  | cu.877774.xyz                         | 104.26.4.111              | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 302  | toy-people.com                        | 2606:4700:20::681a:224    | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 315  | 104.16.223.179                        | 104.16.223.179            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 318  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 319  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9 | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 355  | fbi.gov                               | 2606:4700::6810:95f4      | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 370  | cf.090227.xyz                         | 172.64.152.241            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 389  | eur.877774.xyz                        | 104.21.29.164             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 398  | 104.18.78.214                         | 104.18.78.214             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 400  | 172.67.181.209                        | 172.67.181.209            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 405  | www.digitalocean.com                  | 104.19.174.68             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 406  | www.digitalocean.com                  | 104.19.173.68             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 423  | ifconfig.co                           | 2606:4700:3037::6815:365b | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 427  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8   | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 431  | stock.hostmonit.com                   | 104.21.7.193              | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 436  | japan.com                             | 104.26.5.60               | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 4    | gamer.com.tw                          | 104.18.2.197              | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 14   | time.is                               | 172.67.68.157             | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 17   | time.is                               | 2606:4700:20::681a:d36    | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 42   | cfip.xxxxxxxx.tk                      | 188.114.96.125            | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 44   | www.ipchicken.com                     | 104.26.6.112              | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 45   | www.ipchicken.com                     | 172.67.68.101             | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 46   | www.glassdoor.com                     | 104.16.25.46              | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 51   | ct.877774.xyz                         | 172.64.229.174            | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 74   | ip.sb                                 | 2606:4700:20::681a:d1f    | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 84   | ip.gs                                 | 2606:4700:3035::ac43:a01c | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 85   | singapore.com                         | 104.26.12.140             | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 91   | whatismyipaddress.com                 | 104.19.222.79             | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 97   | 456.cloudflare.182682.xyz             | 104.26.8.160              | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 101  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182            | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 104  | www.visa.com.hk                       | 104.18.21.69              | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 100 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 2 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 2 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
