# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 03:59:51
- **数据来源**: connectivity_results-20251212-035951.json
- **总测试数**: 470
- **失败测试数**: 177
- **成功测试数**: 293
- **失败率**: 37.66%
- **平均延迟**: 101.51ms
- **最小延迟**: 43ms
- **最大延迟**: 811ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 173 次 (97.7%)
- **连接超时: I/O超时**: 4 次 (2.3%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (173 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 455  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 456  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 463  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |
| 467  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 173 次 (97.7%)
- **连接超时**: 4 次 (2.3%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 177 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 173 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：moura.ns.cloudflare.com (3次),
wilson.ns.cloudflare.com (3次), trevor.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 170  | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 283  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 450  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 410  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 42   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 55   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 402  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 424  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 468  | cfip.xxxxxxxx.tk                      | 104.27.21.118   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 470  | cfip.xxxxxxxx.tk                      | 104.17.127.110  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 301  | 456.cloudflare.182682.xyz             | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 414  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 166  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 181  | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 466  | cfip.xxxxxxxx.tk                      | 104.25.105.1    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 175  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 177  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 469  | cfip.xxxxxxxx.tk                      | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 198  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 316  | ae8a9c24-83de.masx200.ddns-ip.net     | 104.21.14.41    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 448  | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 152  | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 219  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 428  | ifconfig.co                           | 104.21.54.91    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 224  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 321  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 64   | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 355  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 423  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 444  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 372  | yx-auto.pages.dev                     | 172.67.161.98   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 7    | yx-auto.pages.dev                     | 104.21.73.250   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 123  | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 197  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 464  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 336  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 272  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 12   | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 380  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 223  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 228  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 404  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 408  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 40   | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 353  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 84   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 398  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 156  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 164  | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 212  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 94   | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 130  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 144  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 169  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 268  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 325  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 80   | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 160  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 297  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 461  | cfip.xxxxxxxx.tk                      | 104.18.228.35   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 41   | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 341  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 367  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 387  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 439  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 190  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 201  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 267  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 290  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 86   | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 258  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 343  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 350  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 433  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 51   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 67   | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 85   | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 136  | yx-auto.pages.dev                     | 172.67.134.139  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 176  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 220  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 2    | www.pcmag.com                         | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 266  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 291  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 311  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 363  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 429  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 102  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 243  | bowen.ns.cloudflare.com               | 162.159.44.83   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 307  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 310  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 312  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 422  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 36   | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 46   | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 61   | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 65   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 127  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 284  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 364  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 395  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 8 条记录
- **快 (50-100ms)**: 92 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 173 次

### 按协议统计

- **none**: 177 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
