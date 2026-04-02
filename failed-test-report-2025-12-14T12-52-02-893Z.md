# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 12:52:02
- **数据来源**: connectivity_results-20251214-125202.json
- **总测试数**: 453
- **失败测试数**: 2
- **成功测试数**: 451
- **失败率**: 0.44%
- **平均延迟**: 50.17ms
- **最小延迟**: 30ms
- **最大延迟**: 818ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 12:52:02
- **IP地址**: 2a09:bac1:7680:28::1d0:dd
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
| 116  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 356  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |

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

| 序号 | 主机/域名                  | 目标IP                    | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | -------------------------- | ------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 410  | ifconfig.co                | 172.67.168.106            | IPv4   | h2   | ✅ 成功 | 30       | cloudflare |
| 375  | 104.18.37.40               | 104.18.37.40              | IPv4   | h2   | ✅ 成功 | 31       | cloudflare |
| 51   | www.4444.cloudflare.182682.xyz                | 162.159.153.2             | IPv4   | h2   | ✅ 成功 | 32       | cloudflare |
| 386  | otto.ns.cloudflare.com     | 108.162.195.135           | IPv4   | h2   | ✅ 成功 | 32       | cloudflare |
| 64   | www.visa.com.sg            | 104.18.13.229             | IPv4   | h2   | ✅ 成功 | 33       | cloudflare |
| 68   | 4444.cloudflare.182682.xyz              | 2606:4700:3033::6815:50b4 | IPv6   | h2   | ✅ 成功 | 33       | cloudflare |
| 105  | pranab.ns.cloudflare.com   | 2606:4700:58::a29f:2cc7   | IPv6   | h2   | ✅ 成功 | 33       | cloudflare |
| 106  | pranab.ns.cloudflare.com   | 2803:f800:50::6ca2:c3c7   | IPv6   | h2   | ✅ 成功 | 33       | cloudflare |
| 159  | cf.0sm.com                 | 2606:4700:3032::6815:785  | IPv6   | h2   | ✅ 成功 | 33       | cloudflare |
| 206  | cloudflare-ip.mofashi.ltd  | 2606:4700:3037::ac43:9bac | IPv6   | h2   | ✅ 成功 | 33       | cloudflare |
| 247  | bowen.ns.cloudflare.com    | 2803:f800:50::6ca2:c353   | IPv6   | h2   | ✅ 成功 | 33       | cloudflare |
| 349  | cf.877774.xyz              | 104.18.41.190             | IPv4   | h2   | ✅ 成功 | 33       | cloudflare |
| 401  | ip.sb                      | 2606:4700:20::681a:d1f    | IPv6   | h2   | ✅ 成功 | 33       | cloudflare |
| 408  | 108.162.198.54             | 108.162.198.54            | IPv4   | h2   | ✅ 成功 | 33       | cloudflare |
| 419  | www.wto.org                | 2a06:98c1:3102::6812:29be | IPv6   | h2   | ✅ 成功 | 33       | cloudflare |
| 437  | japan.com                  | 2606:4700:20::ac43:465c   | IPv6   | h2   | ✅ 成功 | 33       | cloudflare |
| 47   | www.whatismyip.com         | 2606:4700:20::681a:d17    | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 55   | www.hugedomains.com        | 2606:4700:20::ac43:46bf   | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 61   | www.okcupid.com            | 104.16.239.254            | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 74   | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1   | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 110  | cfip.xxxxxxxx.tk           | 104.17.127.110            | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 158  | cf.0sm.com                 | 104.21.7.133              | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 165  | ipinfo.in                  | 2606:4700:3037::ac43:c6cb | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 168  | ipv4.ip.sb                 | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 192  | iplocation.io              | 104.26.10.222             | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 194  | iplocation.io              | 2606:4700:20::681a:bde    | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 205  | cloudflare-ip.mofashi.ltd  | 2606:4700:3037::6815:48e9 | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 213  | cmcc.877774.xyz            | 104.16.149.244            | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 216  | cmcc.877774.xyz            | 104.16.148.3              | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 256  | xn--b6gac.eu.org           | 2606:4700:3035::6815:5a4e | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 259  | saas.sin.fan               | 162.159.36.20             | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 271  | icook.hk                   | 104.21.90.210             | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 301  | rustam.ns.cloudflare.com   | 2606:4700:58::a29f:2c94   | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 315  | 104.18.42.26               | 104.18.42.26              | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 320  | 104.18.14.76               | 104.18.14.76              | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 331  | dnschecker.org             | 172.67.73.216             | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 363  | www.udemy.com              | 2606:4700::6810:8fed      | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 364  | www.udemy.com              | 2606:4700::6810:8eed      | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 400  | ip.sb                      | 2606:4700:20::681a:c1f    | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 411  | ifconfig.co                | 2606:4700:3037::6815:365b | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 429  | lewis.ns.cloudflare.com    | 2606:4700:58::a29f:2c9f   | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 448  | singapore.com              | 104.26.13.140             | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 10   | ip.gs                      | 2606:4700:3036::6815:eb0  | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 23   | 456.cloudflare.182682.xyz  | 104.26.8.160              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 57   | www.hugedomains.com        | 2606:4700:20::681a:625    | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 60   | www.okcupid.com            | 104.16.144.63             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 63   | www.visa.com.sg            | 104.18.12.229             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 81   | cu.877774.xyz              | 104.26.4.119              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 87   | cu.877774.xyz              | 104.26.4.116              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 109  | cfip.xxxxxxxx.tk           | 104.21.91.19              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 117  | cfip.xxxxxxxx.tk           | 190.93.247.169            | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 118  | cfip.xxxxxxxx.tk           | 104.16.241.229            | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 128  | toy-people.com             | 104.26.2.36               | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 153  | www.pcmag.com              | 104.16.20.118             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 156  | 172.67.75.172              | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 167  | ipv4.ip.sb                 | 104.26.12.31              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 169  | ipv4.ip.sb                 | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 173  | steamdb.info               | 2606:4700:10::6814:22d4   | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 178  | www.gov.ua                 | 2606:4700:3031::6815:1748 | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 193  | iplocation.io              | 2606:4700:20::ac43:4664   | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 196  | asia.877774.xyz            | 104.17.139.62             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 202  | zread.ai                   | 2606:4700:3033::6815:4cf0 | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 208  | bestcf.030101.xyz          | 104.19.61.113             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 212  | cmcc.877774.xyz            | 104.16.149.12             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 218  | cmcc.877774.xyz            | 104.16.148.5              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 219  | cmcc.877774.xyz            | 104.16.148.6              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 240  | huxley.ns.cloudflare.com   | 108.162.195.188           | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 249  | bowen.ns.cloudflare.com    | 2606:4700:58::a29f:2c53   | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 252  | cf.zhetengsha.eu.org       | 2606:4700:440a::ac40:98f1 | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 254  | xn--b6gac.eu.org           | 172.67.153.253            | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 262  | www.ipchicken.com          | 172.67.68.101             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 272  | icook.hk                   | 2606:4700:3037::ac43:a168 | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 282  | cf.090227.xyz              | 104.18.42.98              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 283  | cf.090227.xyz              | 2a06:98c1:3105::6812:230f | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 288  | whatismyipaddress.com      | 104.19.223.79             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 307  | time.is                    | 2606:4700:20::ac43:449d   | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 308  | time.is                    | 2606:4700:20::681a:c36    | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 322  | ashton.ns.cloudflare.com   | 172.64.35.173             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 346  | uriah.ns.cloudflare.com    | 2803:f800:50::6ca2:c3c2   | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 351  | cf.877774.xyz              | 2606:4700:4406::ac40:9242 | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 378  | www.digitalocean.com       | 2606:4700::6813:ad44      | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 396  | ip.sb                      | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 397  | ip.sb                      | 104.26.12.31              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 398  | ip.sb                      | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 404  | benedict.ns.cloudflare.com | 108.162.195.205           | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 415  | www.glassdoor.com          | 104.17.64.70              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 422  | stock.hostmonit.com        | 2606:4700:3033::ac43:bbfb | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 432  | japan.com                  | 104.26.4.60               | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 438  | palera.in                  | 172.67.157.122            | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 440  | palera.in                  | 2606:4700:3035::6815:3a48 | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 441  | palera.in                  | 2606:4700:3032::ac43:9d7a | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 453  | singapore.com              | 2606:4700:20::681a:c8c    | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 11   | ip.gs                      | 2606:4700:3035::ac43:a01c | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 14   | comicabc.com               | 2606:4700:3036::6815:400a | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 24   | 456.cloudflare.182682.xyz  | 104.26.9.160              | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 49   | 172.67.120.0               | 172.67.120.0              | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 65   | 4444.cloudflare.182682.xyz              | 104.21.80.180             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 67   | 4444.cloudflare.182682.xyz              | 2606:4700:3033::ac43:98b7 | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 75   | 172.67.243.218             | 172.67.243.218            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 79   | yx-auto.pages.dev          | 2606:4700:310c::ac42:2f70 | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |

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
