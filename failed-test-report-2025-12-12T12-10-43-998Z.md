# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 12:10:43
- **数据来源**: connectivity_results-20251212-121043.json
- **总测试数**: 445
- **失败测试数**: 168
- **成功测试数**: 277
- **失败率**: 37.75%
- **平均延迟**: 105.56ms
- **最小延迟**: 50ms
- **最大延迟**: 724ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 12:10:44
- **IP地址**: 20.168.109.194
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

- **网络不可达: 网络不可达**: 165 次 (98.2%)
- **连接超时: I/O超时**: 3 次 (1.8%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (165 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 439  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 444  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 445  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 165 次 (98.2%)
- **连接超时**: 3 次 (1.8%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 168 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 165 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：time.is (3次), wilson.ns.cloudflare.com
(3次), rustam.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 215  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 264  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 368  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 138  | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 161  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 311  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 413  | cf.090227.xyz                         | 104.18.42.98    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 188  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 262  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 345  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 306  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 363  | dylan.ns.cloudflare.com               | 162.159.44.187  | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 146  | cu.877774.xyz                         | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 358  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 400  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 325  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 412  | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 289  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 226  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 34   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 235  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 300  | abdullah.ns.cloudflare.com            | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 130  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 238  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 374  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 411  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 38   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 109  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 261  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 61   | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 67   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 119  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 254  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 274  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 276  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 35   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 76   | icook.hk                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 174  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 295  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 15   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 47   | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 68   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 72   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 97   | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 223  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 249  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 286  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 290  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 11   | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 16   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 65   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 268  | lewis.ns.cloudflare.com               | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 299  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 112  | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 285  | ifconfig.co                           | 104.21.54.91    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 310  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 314  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 417  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 118  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 120  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 143  | cu.877774.xyz                         | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 162  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 203  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 209  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 317  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 23   | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 46   | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 80   | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 141  | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 191  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 236  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 250  | otto.ns.cloudflare.com                | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 6    | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 90   | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 91   | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 211  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 255  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 318  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 432  | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 167  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 175  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 196  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 244  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 256  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 281  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 324  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 369  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 42   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 110  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 302  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 335  | craig.ns.cloudflare.com               | 162.159.44.192  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 356  | kyree.ns.cloudflare.com               | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 33   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 41   | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 204  | julio.ns.cloudflare.com               | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 320  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 357  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 376  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 385  | bestcf.030101.xyz                     | 104.17.27.231   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 401  | braden.ns.cloudflare.com              | 162.159.44.169  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 100 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 165 次

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
