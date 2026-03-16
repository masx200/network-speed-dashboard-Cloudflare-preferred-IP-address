# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 12:55:02
- **数据来源**: connectivity_results-20251214-125502.json
- **总测试数**: 454
- **失败测试数**: 168
- **成功测试数**: 286
- **失败率**: 37.00%
- **平均延迟**: 103.66ms
- **最小延迟**: 57ms
- **最大延迟**: 520ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 12:55:02
- **IP地址**: 172.212.169.145
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: cloud
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 41.6015, -93.6127
- **时区**: America/Chicago
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 164 次 (97.6%)
- **连接超时: I/O超时**: 4 次 (2.4%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (164 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 376  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 445  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 451  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |
| 454  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 164 次 (97.6%)
- **连接超时**: 4 次 (2.4%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 168 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 164 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：wilson.ns.cloudflare.com (3次),
iplocation.io (3次), huxley.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 270  | 456.cloudflare.182682.xyz             | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 366  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 272  | 456.cloudflare.182682.xyz             | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 133  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 128  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 421  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 201  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 405  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 416  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 449  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 170  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 222  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 364  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 94   | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 301  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 258  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 52   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 427  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 286  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 346  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 58   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 68   | cu.877774.xyz                         | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 253  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 284  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 353  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 413  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 359  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 48   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 157  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 199  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 267  | freeyx.cloudflare88.eu.org            | 141.101.120.207 | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 276  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 195  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 261  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 419  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 426  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 95   | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 182  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 249  | cf.zhetengsha.eu.org                  | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 332  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 418  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 446  | cfip.xxxxxxxx.tk                      | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 34   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 145  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 159  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 236  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 256  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 37   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 49   | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 67   | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 164  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 205  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 212  | cf.090227.xyz                         | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 280  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 300  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 340  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 372  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 386  | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 447  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 33   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 75   | www.7749tv.com                        | 104.19.93.88    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 118  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 119  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 243  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 302  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 328  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 335  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 336  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 341  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 377  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 425  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 440  | cfip.xxxxxxxx.tk                      | 104.17.127.110  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 444  | cfip.xxxxxxxx.tk                      | 104.18.228.35   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 452  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 99   | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 155  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 216  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 235  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 293  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 337  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 338  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 378  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 393  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 450  | cfip.xxxxxxxx.tk                      | 188.114.97.144  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 27   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 39   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 45   | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 73   | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 137  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 163  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 370  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 432  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 435  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 437  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 26   | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 36   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 59   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 77   | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 234  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 277  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 100 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 164 次

### 按协议统计

- **none**: 168 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
