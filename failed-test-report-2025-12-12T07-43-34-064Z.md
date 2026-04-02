# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 15:43:34
- **数据来源**: connectivity_results-20251212-154327.json
- **总测试数**: 463
- **失败测试数**: 7
- **成功测试数**: 456
- **失败率**: 1.51%
- **平均延迟**: 749.82ms
- **最小延迟**: 146ms
- **最大延迟**: 9249ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 15:43:34
- **IP地址**: 2409:891f:8223:a9e6:b8fa:1abc:1a83:143b
- **国家/地区**: China (CN)
- **ASN**: AS24400
- **网络组织**: Shanghai Mobile Communications Co.,Ltd.
- **网络域名**: chinamobile.com
- **大洲**: Asia (AS)
- **地理坐标**: 34.7732, 113.722
- **时区**: Asia/Shanghai
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 6 次 (85.7%)
- **DNS解析错误: 其他DNS错误**: 1 次 (14.3%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (6 次测试)

| 序号 | 主机/域名                | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ------------------------ | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 141  | cfip.xxxxxxxx.tk         | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 149  | trevor.ns.cloudflare.com | 4444.cloudflare.182682.xyz | IPv4   | none | N/A    | 0        | N/A    | dial tcp 4444.cloudflare.182682.xyz:443: i/o timeout |
| 202  | cf.877774.xyz            | cf.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout   |
| 367  | 141.147.185.63           | 141.147.185.63  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 141.147.185.63:443: i/o timeout  |
| 451  | ct.877774.xyz            | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 463  | 3.0.50.69                | 3.0.50.69       | IPv4   | none | N/A    | 0        | N/A    | dial tcp 3.0.50.69:443: i/o timeout       |

#### DNS解析错误: 其他DNS错误 (1 次测试)

| 序号 | 主机/域名        | 目标IP  | IP版本  | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息      |
| ---- | ---------------- | ------- | ------- | ---- | ------ | -------- | ------ | ------------- |
| 390  | cfip.1323123.xyz | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析无结果 |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 6 次 (85.7%)
- **DNS解析错误**: 1 次 (14.3%)

#### 错误模式分析

**超时集中度分析**: 共有 6 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 7 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                  | 目标IP                   | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | -------------------------- | ------------------------ | ------ | ---- | ------- | -------- | ---------- |
| 4    | ct.877774.xyz              | 172.64.229.174           | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 252  | cf.zhetengsha.eu.org       | 172.64.144.82            | IPv4   | h3   | ✅ 成功 | 155      | cloudflare |
| 51   | cmcc.877774.xyz            | 104.16.149.8             | IPv4   | h3   | ✅ 成功 | 156      | cloudflare |
| 35   | cmcc.877774.xyz            | 104.16.148.5             | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 251  | cf.zhetengsha.eu.org       | 104.18.43.174            | IPv4   | h3   | ✅ 成功 | 161      | cloudflare |
| 5    | ct.877774.xyz              | 172.64.229.185           | IPv4   | h3   | ✅ 成功 | 162      | cloudflare |
| 240  | fbi.gov                    | 104.16.148.244           | IPv4   | h3   | ✅ 成功 | 162      | cloudflare |
| 9    | ct.877774.xyz              | 172.64.229.44            | IPv4   | h3   | ✅ 成功 | 164      | cloudflare |
| 33   | cmcc.877774.xyz            | 104.16.148.3             | IPv4   | h3   | ✅ 成功 | 164      | cloudflare |
| 36   | cmcc.877774.xyz            | 104.16.148.6             | IPv4   | h3   | ✅ 成功 | 166      | cloudflare |
| 37   | cmcc.877774.xyz            | 104.16.148.7             | IPv4   | h3   | ✅ 成功 | 167      | cloudflare |
| 29   | cmcc.877774.xyz            | 104.16.149.12            | IPv4   | h3   | ✅ 成功 | 168      | cloudflare |
| 241  | fbi.gov                    | 104.16.149.244           | IPv4   | h3   | ✅ 成功 | 168      | cloudflare |
| 38   | cmcc.877774.xyz            | 104.16.148.8             | IPv4   | h3   | ✅ 成功 | 169      | cloudflare |
| 233  | cf.090227.xyz              | 104.18.42.98             | IPv4   | h3   | ✅ 成功 | 169      | cloudflare |
| 3    | ct.877774.xyz              | 172.64.229.173           | IPv4   | h3   | ✅ 成功 | 171      | cloudflare |
| 31   | cmcc.877774.xyz            | 104.16.148.1             | IPv4   | h3   | ✅ 成功 | 171      | cloudflare |
| 44   | cmcc.877774.xyz            | 104.16.149.1             | IPv4   | h3   | ✅ 成功 | 171      | cloudflare |
| 32   | cmcc.877774.xyz            | 104.16.148.2             | IPv4   | h3   | ✅ 成功 | 175      | cloudflare |
| 34   | cmcc.877774.xyz            | 104.16.148.4             | IPv4   | h3   | ✅ 成功 | 175      | cloudflare |
| 46   | cmcc.877774.xyz            | 104.16.149.3             | IPv4   | h3   | ✅ 成功 | 175      | cloudflare |
| 138  | cfip.xxxxxxxx.tk           | 104.16.241.229           | IPv4   | h3   | ✅ 成功 | 175      | cloudflare |
| 39   | cmcc.877774.xyz            | 104.16.148.9             | IPv4   | h3   | ✅ 成功 | 176      | cloudflare |
| 6    | ct.877774.xyz              | 172.64.229.195           | IPv4   | h3   | ✅ 成功 | 180      | cloudflare |
| 52   | cmcc.877774.xyz            | 104.16.149.9             | IPv4   | h3   | ✅ 成功 | 180      | cloudflare |
| 28   | cmcc.877774.xyz            | 104.16.149.11            | IPv4   | h3   | ✅ 成功 | 181      | cloudflare |
| 232  | cf.090227.xyz              | 172.64.145.158           | IPv4   | h3   | ✅ 成功 | 182      | cloudflare |
| 8    | ct.877774.xyz              | 172.64.229.236           | IPv4   | h3   | ✅ 成功 | 183      | cloudflare |
| 41   | cmcc.877774.xyz            | 104.16.148.11            | IPv4   | h3   | ✅ 成功 | 184      | cloudflare |
| 103  | www.okcupid.com            | 104.16.144.63            | IPv4   | h3   | ✅ 成功 | 185      | cloudflare |
| 49   | cmcc.877774.xyz            | 104.16.149.6             | IPv4   | h3   | ✅ 成功 | 186      | cloudflare |
| 459  | abdullah.ns.cloudflare.com | 172.64.35.203            | IPv4   | h3   | ✅ 成功 | 189      | cloudflare |
| 48   | cmcc.877774.xyz            | 104.16.149.5             | IPv4   | h3   | ✅ 成功 | 190      | cloudflare |
| 102  | www.okcupid.com            | 104.17.48.63             | IPv4   | h3   | ✅ 成功 | 190      | cloudflare |
| 211  | bestcf.030101.xyz          | 104.19.33.117            | IPv4   | h3   | ✅ 成功 | 192      | cloudflare |
| 329  | 104.18.42.26               | 104.18.42.26             | IPv4   | h3   | ✅ 成功 | 194      | cloudflare |
| 47   | cmcc.877774.xyz            | 104.16.149.4             | IPv4   | h3   | ✅ 成功 | 199      | cloudflare |
| 53   | cmcc.877774.xyz            | 104.16.149.10            | IPv4   | h3   | ✅ 成功 | 199      | cloudflare |
| 397  | 104.19.223.58              | 104.19.223.58            | IPv4   | h3   | ✅ 成功 | 199      | cloudflare |
| 210  | bestcf.030101.xyz          | 104.19.148.231           | IPv4   | h3   | ✅ 成功 | 200      | cloudflare |
| 43   | cmcc.877774.xyz            | 104.16.148.244           | IPv4   | h3   | ✅ 成功 | 202      | cloudflare |
| 30   | cmcc.877774.xyz            | 104.16.149.244           | IPv4   | h3   | ✅ 成功 | 204      | cloudflare |
| 42   | cmcc.877774.xyz            | 104.16.148.12            | IPv4   | h3   | ✅ 成功 | 205      | cloudflare |
| 110  | na.877774.xyz              | 104.18.38.235            | IPv4   | h3   | ✅ 成功 | 206      | cloudflare |
| 7    | ct.877774.xyz              | 172.64.229.217           | IPv4   | h3   | ✅ 成功 | 207      | cloudflare |
| 377  | 104.17.142.12              | 104.17.142.12            | IPv4   | h3   | ✅ 成功 | 210      | cloudflare |
| 168  | craig.ns.cloudflare.com    | 108.162.195.192          | IPv4   | h3   | ✅ 成功 | 215      | cloudflare |
| 324  | whatismyipaddress.com      | 104.19.223.79            | IPv4   | h3   | ✅ 成功 | 221      | cloudflare |
| 105  | www.okcupid.com            | 104.18.160.63            | IPv4   | h3   | ✅ 成功 | 223      | cloudflare |
| 113  | sullivan.ns.cloudflare.com | 108.162.195.161          | IPv4   | h3   | ✅ 成功 | 223      | cloudflare |
| 10   | ct.877774.xyz              | 172.64.229.161           | IPv4   | h3   | ✅ 成功 | 224      | cloudflare |
| 260  | bowen.ns.cloudflare.com    | 108.162.195.83           | IPv4   | h3   | ✅ 成功 | 224      | cloudflare |
| 323  | whatismyipaddress.com      | 104.19.222.79            | IPv4   | h3   | ✅ 成功 | 228      | cloudflare |
| 45   | cmcc.877774.xyz            | 104.16.149.2             | IPv4   | h3   | ✅ 成功 | 229      | cloudflare |
| 203  | asia.877774.xyz            | 104.17.142.146           | IPv4   | h3   | ✅ 成功 | 232      | cloudflare |
| 216  | dylan.ns.cloudflare.com    | 172.64.35.187            | IPv4   | h3   | ✅ 成功 | 233      | cloudflare |
| 279  | rustam.ns.cloudflare.com   | 108.162.195.148          | IPv4   | h3   | ✅ 成功 | 234      | cloudflare |
| 50   | cmcc.877774.xyz            | 104.16.149.7             | IPv4   | h3   | ✅ 成功 | 236      | cloudflare |
| 120  | www.4444.cloudflare.182682.xyz                | 162.159.152.2            | IPv4   | h3   | ✅ 成功 | 242      | cloudflare |
| 151  | trevor.ns.cloudflare.com   | 172.64.35.154            | IPv4   | h3   | ✅ 成功 | 245      | cloudflare |
| 364  | cf.877774.xyz              | 172.64.146.66            | IPv4   | h3   | ✅ 成功 | 245      | cloudflare |
| 162  | cris.ns.cloudflare.com     | 108.162.195.202          | IPv4   | h3   | ✅ 成功 | 248      | cloudflare |
| 431  | 108.162.198.54             | 108.162.198.54           | IPv4   | h3   | ✅ 成功 | 253      | cloudflare |
| 17   | wilson.ns.cloudflare.com   | 172.64.35.110            | IPv4   | h3   | ✅ 成功 | 270      | cloudflare |
| 293  | benedict.ns.cloudflare.com | 172.64.35.205            | IPv4   | h3   | ✅ 成功 | 273      | cloudflare |
| 81   | 34.143.159.175             | 34.143.159.175           | IPv4   | h2   | ✅ 成功 | 306      | cloudflare |
| 40   | cmcc.877774.xyz            | 104.16.148.10            | IPv4   | h3   | ✅ 成功 | 322      | cloudflare |
| 119  | www.4444.cloudflare.182682.xyz                | 162.159.153.2            | IPv4   | h3   | ✅ 成功 | 325      | cloudflare |
| 87   | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6   | h3   | ✅ 成功 | 328      | cloudflare |
| 303  | singapore.com              | 104.26.13.140            | IPv4   | h3   | ✅ 成功 | 351      | cloudflare |
| 111  | na.877774.xyz              | 104.18.187.25            | IPv4   | h3   | ✅ 成功 | 358      | cloudflare |
| 199  | www.whatismyip.com         | 2606:4700:20::681a:c17   | IPv6   | h3   | ✅ 成功 | 381      | cloudflare |
| 129  | cu.877774.xyz              | 104.26.4.115             | IPv4   | h3   | ✅ 成功 | 382      | cloudflare |
| 441  | otto.ns.cloudflare.com     | 108.162.195.135          | IPv4   | h3   | ✅ 成功 | 385      | cloudflare |
| 228  | decker.ns.cloudflare.com   | 172.64.35.155            | IPv4   | h3   | ✅ 成功 | 390      | cloudflare |
| 448  | yx-auto.pages.dev          | 172.67.151.207           | IPv4   | h3   | ✅ 成功 | 394      | cloudflare |
| 430  | 104.19.175.123             | 104.19.175.123           | IPv4   | h3   | ✅ 成功 | 398      | cloudflare |
| 256  | www.glassdoor.com          | 104.17.64.70             | IPv4   | h3   | ✅ 成功 | 400      | cloudflare |
| 353  | 162.159.133.85             | 162.159.133.85           | IPv4   | h3   | ✅ 成功 | 403      | cloudflare |
| 316  | 456.cloudflare.182682.xyz  | 2606:4700:20::681a:8a0   | IPv6   | h3   | ✅ 成功 | 404      | cloudflare |
| 368  | yx-auto.pages.dev          | 172.67.161.98            | IPv4   | h3   | ✅ 成功 | 407      | cloudflare |
| 301  | singapore.com              | 104.26.12.140            | IPv4   | h3   | ✅ 成功 | 411      | cloudflare |
| 205  | asia.877774.xyz            | 104.17.139.62            | IPv4   | h3   | ✅ 成功 | 414      | cloudflare |
| 174  | www.visa.com.sg            | 104.18.12.229            | IPv4   | h3   | ✅ 成功 | 415      | cloudflare |
| 399  | www.digitalocean.com       | 104.19.174.68            | IPv4   | h3   | ✅ 成功 | 420      | cloudflare |
| 317  | time.is                    | 104.26.13.54             | IPv4   | h3   | ✅ 成功 | 421      | cloudflare |
| 391  | 104.26.13.31               | 104.26.13.31             | IPv4   | h3   | ✅ 成功 | 425      | cloudflare |
| 180  | toy-people.com             | 172.67.72.18             | IPv4   | h3   | ✅ 成功 | 428      | cloudflare |
| 121  | cu.877774.xyz              | 104.26.4.116             | IPv4   | h3   | ✅ 成功 | 430      | cloudflare |
| 54   | www.gov.ua                 | 172.67.209.127           | IPv4   | h3   | ✅ 成功 | 431      | cloudflare |
| 276  | ip.sb                      | 2606:4700:20::681a:c1f   | IPv6   | h3   | ✅ 成功 | 432      | cloudflare |
| 278  | 104.16.61.163              | 104.16.61.163            | IPv4   | h3   | ✅ 成功 | 433      | cloudflare |
| 258  | www.ipchicken.com          | 104.26.7.112             | IPv4   | h3   | ✅ 成功 | 434      | cloudflare |
| 318  | time.is                    | 104.26.12.54             | IPv4   | h3   | ✅ 成功 | 438      | cloudflare |
| 457  | abdullah.ns.cloudflare.com | 108.162.195.203          | IPv4   | h3   | ✅ 成功 | 439      | cloudflare |
| 392  | 104.18.78.214              | 104.18.78.214            | IPv4   | h3   | ✅ 成功 | 440      | cloudflare |
| 286  | 104.17.68.85               | 104.17.68.85             | IPv4   | h3   | ✅ 成功 | 441      | cloudflare |
| 16   | wilson.ns.cloudflare.com   | 162.159.44.110           | IPv4   | h3   | ✅ 成功 | 442      | cloudflare |
| 131  | www.hugedomains.com        | 104.26.7.37              | IPv4   | h3   | ✅ 成功 | 442      | cloudflare |
| 272  | ip.sb                      | 172.67.75.172            | IPv4   | h3   | ✅ 成功 | 442      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 39 条记录
- **慢 (200-500ms)**: 61 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 6 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 7 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
