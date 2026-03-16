# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 07:08:23
- **数据来源**: connectivity_results-20251212-070823.json
- **总测试数**: 461
- **失败测试数**: 2
- **成功测试数**: 459
- **失败率**: 0.43%
- **平均延迟**: 60.64ms
- **最小延迟**: 36ms
- **最大延迟**: 582ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 07:08:23
- **IP地址**: 2a09:bac5:6211:1250::1d3:9e
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
| 20   | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 391  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

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
| 133  | www.visa.com.sg                       | 104.18.13.229             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 56   | cf.0sm.com                            | 172.67.187.145            | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 189  | zread.ai                              | 2606:4700:3033::6815:4cf0 | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 126  | www.hugedomains.com                   | 104.26.7.37               | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 190  | zread.ai                              | 2606:4700:3032::ac43:ca4e | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 64   | 104.16.45.84                          | 104.16.45.84              | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 103  | icook.hk                              | 172.67.161.104            | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 119  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7 | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 128  | www.hugedomains.com                   | 104.26.6.37               | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 266  | fbi.gov                               | 104.16.148.244            | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 274  | cf.090227.xyz                         | 2606:4700:440a::ac40:98f1 | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 285  | palera.in                             | 2606:4700:3032::ac43:9d7a | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 319  | singapore.com                         | 2606:4700:20::ac43:4bc2   | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 354  | dnschecker.org                        | 2606:4700:20::681a:659    | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 388  | www.udemy.com                         | 104.16.143.237            | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 443  | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1  | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 448  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f   | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 19   | www.ipget.net                         | 2606:4700:3031::ac43:cf1a | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 26   | time.is                               | 2606:4700:20::681a:d36    | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 53   | www.pcmag.com                         | 104.16.20.118             | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 57   | cf.0sm.com                            | 104.21.7.133              | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 63   | ipv4.ip.sb                            | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 65   | shopify.com                           | 23.227.38.33              | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 66   | 172.67.75.172                         | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 106  | icook.hk                              | 2606:4700:3037::ac43:a168 | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 139  | sullivan.ns.cloudflare.com            | 2803:f800:50::6ca2:c3a1   | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 243  | cmcc.877774.xyz                       | 104.16.148.11             | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 260  | cf.zhetengsha.eu.org                  | 2a06:98c1:3101::ac40:919e | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 317  | singapore.com                         | 104.26.13.140             | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 321  | whatismyipaddress.com                 | 104.19.222.79             | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 324  | whatismyipaddress.com                 | 2606:4700::6813:df4f      | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 333  | 104.18.42.26                          | 104.18.42.26              | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 336  | local-aria2-webui.masx200.ddns-ip.net | 104.21.14.41              | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 337  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182            | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 340  | 104.18.14.76                          | 104.18.14.76              | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 356  | dnschecker.org                        | 2606:4700:20::ac43:49d8   | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 384  | tasteatlas.com                        | 104.17.36.105             | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 385  | tasteatlas.com                        | 2606:4700::6811:2469      | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 429  | www.csgo.com                          | 195.85.59.161             | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 438  | www.wto.org                           | 2606:4700:4406::ac40:9242 | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 439  | www.wto.org                           | 2a06:98c1:3102::6812:29be | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 454  | japan.com                             | 2606:4700:20::681a:53c    | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 15   | yx-auto.pages.dev                     | 2606:4700:3034::ac43:97cf | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 62   | ipv4.ip.sb                            | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 73   | steamdb.info                          | 2606:4700:10::6814:22d4   | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 79   | 104.18.37.13                          | 104.18.37.13              | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 85   | ct.877774.xyz                         | 172.64.229.44             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 101  | www.4chan.org                         | 104.16.229.229            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 102  | www.4chan.org                         | 104.16.228.229            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 105  | icook.hk                              | 2606:4700:3031::6815:5ad2 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 116  | www.visa.cn                           | 162.159.152.2             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 141  | yx-auto.pages.dev                     | 104.21.6.60               | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 178  | freeyx.cloudflare88.eu.org            | 141.101.120.112           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 212  | cloudflare-ip.mofashi.ltd             | 172.67.155.172            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 235  | cmcc.877774.xyz                       | 104.16.148.3              | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 238  | cmcc.877774.xyz                       | 104.16.148.6              | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 264  | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 296  | ae8a9c24-83de.masx200.ddns-ip.net     | 2606:4700:3031::ac43:9db6 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 300  | ip.sb                                 | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 316  | singapore.com                         | 104.26.12.140             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 322  | whatismyipaddress.com                 | 104.19.223.79             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 330  | 456.cloudflare.182682.xyz                          | 2606:4700:20::ac43:4bd0   | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 344  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 380  | yx-auto.pages.dev                     | 2606:4700:3034::6815:9e6  | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 400  | icook.tw                              | 2606:4700:10::ac42:9e73   | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 405  | www.digitalocean.com                  | 104.19.174.68             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 409  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47  | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 436  | www.wto.org                           | 172.64.146.66             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 452  | japan.com                             | 104.26.4.60               | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 14   | yx-auto.pages.dev                     | 2606:4700:3031::6815:49fa | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 18   | www.ipget.net                         | 2606:4700:3036::6815:fd4  | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 58   | cf.0sm.com                            | 2606:4700:3032::6815:785  | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 70   | ipinfo.in                             | 2606:4700:3037::ac43:c6cb | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 89   | ct.877774.xyz                         | 172.64.229.195            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 93   | 104.18.254.88                         | 104.18.254.88             | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 97   | iplocation.io                         | 172.67.70.100             | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 107  | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 120  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4 | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 125  | www.okcupid.com                       | 104.18.160.63             | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 127  | www.hugedomains.com                   | 172.67.70.191             | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 132  | www.visa.com.sg                       | 104.18.12.229             | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 159  | cu.877774.xyz                         | 104.26.4.117              | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 160  | na.877774.xyz                         | 104.18.38.235             | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 208  | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb   | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 220  | www.whatismyip.com                    | 2606:4700:20::681a:d17    | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 239  | cmcc.877774.xyz                       | 104.16.148.7              | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 241  | cmcc.877774.xyz                       | 104.16.148.9              | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 249  | cmcc.877774.xyz                       | 104.16.149.4              | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 265  | xn--b6gac.eu.org                      | 2606:4700:3037::ac43:99fd | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 268  | fbi.gov                               | 2606:4700::6810:95f4      | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 302  | ip.sb                                 | 2606:4700:20::681a:d1f    | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 313  | ip.gs                                 | 2606:4700:3035::ac43:a01c | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 323  | whatismyipaddress.com                 | 2606:4700::6813:de4f      | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 327  | 456.cloudflare.182682.xyz                          | 104.26.8.160              | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 334  | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 355  | dnschecker.org                        | 2606:4700:20::681a:759    | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 359  | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 368  | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06 | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 379  | yx-auto.pages.dev                     | 104.21.9.230              | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 401  | icook.tw                              | 2606:4700:10::6814:1c4a   | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |

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
