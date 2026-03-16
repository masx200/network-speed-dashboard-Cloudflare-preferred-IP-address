# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 15:14:21
- **数据来源**: connectivity_results-20251212-151421.json
- **总测试数**: 456
- **失败测试数**: 168
- **成功测试数**: 288
- **失败率**: 36.84%
- **平均延迟**: 129.82ms
- **最小延迟**: 45ms
- **最大延迟**: 618ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 15:14:21
- **IP地址**: 172.208.126.100
- **国家/地区**: United States (US)
- **ASN**: AS8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **数据源**: ipinfo.io

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
| 441  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 442  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 447  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 449  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |

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
trevor.ns.cloudflare.com (3次), iplocation.io
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                  | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | -------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 262  | 104.16.223.179             | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 73   | cf.877771.xyz              | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 389  | time.is                    | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 149  | 172.67.79.211              | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 139  | cmcc.877774.xyz            | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 229  | icook.tw                   | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 455  | cfip.xxxxxxxx.tk           | 190.93.247.169  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 384  | moura.ns.cloudflare.com    | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 156  | 104.17.68.85               | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 222  | www.udemy.com              | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 134  | cmcc.877774.xyz            | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 146  | na.877774.xyz              | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 185  | toy-people.com             | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 414  | ip.sb                      | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 281  | saas.sin.fan               | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 17   | comicabc.com               | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 202  | 104.18.14.76               | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 226  | gamer.com.tw               | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 435  | dylan.ns.cloudflare.com    | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 452  | cfip.xxxxxxxx.tk           | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 318  | eur.877774.xyz             | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 98   | www.visa.cn                | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 175  | dnschecker.org             | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 280  | saas.sin.fan               | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 136  | cmcc.877774.xyz            | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 406  | singapore.com              | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 85   | www.okcupid.com            | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 235  | freeyx.cloudflare88.eu.org | 141.101.120.7   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 395  | rustam.ns.cloudflare.com   | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 326  | 104.18.37.13               | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 36   | steamdb.info               | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 169  | ashton.ns.cloudflare.com   | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 258  | damien.ns.cloudflare.com   | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 283  | zread.ai                   | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 330  | cf.zhetengsha.eu.org       | 172.64.145.158  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 454  | cfip.xxxxxxxx.tk           | 104.17.127.110  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 83   | www.okcupid.com            | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 106  | 172.67.243.218             | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 212  | 162.159.133.85             | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 292  | www.whatismyip.com         | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 378  | benedict.ns.cloudflare.com | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 33   | www.ipget.net              | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 95   | cu.877774.xyz              | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 347  | 172.67.181.209             | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 370  | 172.64.151.55              | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 412  | ip.sb                      | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 21   | www.pcmag.com              | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 111  | www.7749tv.com             | 104.16.10.137   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 148  | na.877774.xyz              | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 234  | freeyx.cloudflare88.eu.org | 141.101.120.23  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 320  | lewis.ns.cloudflare.com    | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 79   | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 138  | cmcc.877774.xyz            | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 140  | cmcc.877774.xyz            | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 291  | www.whatismyip.com         | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 88   | cu.877774.xyz              | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 121  | cmcc.877774.xyz            | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 179  | decker.ns.cloudflare.com   | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 265  | bestcf.030101.xyz          | 104.19.153.222  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 401  | www.ipchicken.com          | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 429  | 456.cloudflare.182682.xyz  | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 56   | ipv4.ip.sb                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 451  | cfip.xxxxxxxx.tk           | 198.41.214.141  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 201  | 172.67.106.26              | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 453  | cfip.xxxxxxxx.tk           | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 51   | www.gov.ua                 | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 96   | cu.877774.xyz              | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 67   | www.hugedomains.com        | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 102  | craig.ns.cloudflare.com    | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 307  | bowen.ns.cloudflare.com    | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 317  | eur.877774.xyz             | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 43   | ct.877774.xyz              | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 46   | ct.877774.xyz              | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 218  | tasteatlas.com             | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 16   | comicabc.com               | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 20   | www.pcmag.com              | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 158  | whatismyipaddress.com      | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 204  | uriah.ns.cloudflare.com    | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 65   | www.4chan.org              | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 94   | cu.877774.xyz              | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 89   | cu.877774.xyz              | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 126  | cmcc.877774.xyz            | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 304  | 104.18.37.40               | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 419  | ip.gs                      | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 91   | cu.877774.xyz              | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 157  | whatismyipaddress.com      | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 264  | bestcf.030101.xyz          | 104.17.222.192  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 48   | 103.160.204.59             | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 55   | ipv4.ip.sb                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 141  | cmcc.877774.xyz            | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 239  | 104.17.142.12              | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 405  | singapore.com              | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 47   | ct.877774.xyz              | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 78   | sullivan.ns.cloudflare.com | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 152  | cris.ns.cloudflare.com     | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 217  | tasteatlas.com             | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 230  | icook.tw                   | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 306  | 172.64.35.24               | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 54   | ipv4.ip.sb                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 113  | www.visa.com.sg            | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 1 条记录
- **快 (50-100ms)**: 60 条记录
- **正常 (100-200ms)**: 39 条记录
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
