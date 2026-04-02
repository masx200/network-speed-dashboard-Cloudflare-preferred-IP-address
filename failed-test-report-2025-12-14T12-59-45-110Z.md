# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 12:59:45
- **数据来源**: connectivity_results-20251214-125943.json
- **总测试数**: 452
- **失败测试数**: 6
- **成功测试数**: 446
- **失败率**: 1.33%
- **平均延迟**: 123.61ms
- **最小延迟**: 58ms
- **最大延迟**: 788ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 12:59:45
- **IP地址**: 2a09:bac1:76e0:8e0::1bd:c6
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 33.4475, -112.0866
- **时区**: America/Phoenix
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 6 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (6 次测试)

| 序号 | 主机/域名                  | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | -------------------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 118  | ct.877774.xyz              | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 240  | abdullah.ns.cloudflare.com | 162.159.44.203  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 162.159.44.203:443: i/o timeout  |
| 288  | huxley.ns.cloudflare.com   | 172.64.35.188   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.35.188:443: i/o timeout   |
| 334  | cfip.xxxxxxxx.tk           | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 359  | pranab.ns.cloudflare.com   | 162.159.44.199  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 162.159.44.199:443: i/o timeout  |
| 380  | cris.ns.cloudflare.com     | 108.162.195.202 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.202:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 6 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 6 次超时，主要集中在IP段 172.64（2
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 6 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP                    | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | ------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 76   | www.visa.com.hk                       | 104.18.20.69              | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 107  | cf.877774.xyz                         | 172.64.146.66             | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 17   | palera.in                             | 2606:4700:3035::6815:3a48 | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 47   | ip.sb                                 | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 52   | ip.gs                                 | 188.114.96.3              | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 53   | ip.gs                                 | 188.114.97.3              | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 72   | whatismyipaddress.com                 | 104.19.222.79             | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 16   | palera.in                             | 104.21.58.72              | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 28   | iplocation.io                         | 2606:4700:20::ac43:4664   | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 30   | iplocation.io                         | 2606:4700:20::681a:ade    | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 46   | ip.sb                                 | 104.26.12.31              | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 49   | ip.sb                                 | 2606:4700:20::ac43:4bac   | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 70   | 104.18.42.26                          | 104.18.42.26              | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 78   | 172.67.106.26                         | 172.67.106.26             | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 27   | iplocation.io                         | 172.67.70.100             | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 42   | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd   | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 45   | ip.sb                                 | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 18   | palera.in                             | 2606:4700:3032::ac43:9d7a | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 23   | trevor.ns.cloudflare.com              | 2606:4700:58::a29f:2c9a   | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 50   | ip.sb                                 | 2606:4700:20::681a:c1f    | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 56   | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 62   | singapore.com                         | 2606:4700:20::ac43:4bc2   | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 79   | 104.18.14.76                          | 104.18.14.76              | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 96   | dnschecker.org                        | 104.26.7.89               | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 103  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2   | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 189  | cmcc.877774.xyz                       | 104.16.148.9              | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 192  | cmcc.877774.xyz                       | 104.16.148.6              | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 200  | cmcc.877774.xyz                       | 104.16.149.11             | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 22   | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a   | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 26   | iplocation.io                         | 104.26.11.222             | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 29   | iplocation.io                         | 2606:4700:20::681a:bde    | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 37   | 104.16.61.163                         | 104.16.61.163             | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 54   | ip.gs                                 | 2606:4700:3035::ac43:a01c | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 67   | 456.cloudflare.182682.xyz             | 2606:4700:20::681a:8a0    | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 68   | 456.cloudflare.182682.xyz             | 2606:4700:20::ac43:4bd0   | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 4    | www.7749tv.com                        | 104.19.93.88              | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 12   | comicabc.com                          | 188.114.97.3              | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 34   | wilson.ns.cloudflare.com              | 2606:4700:58::a29f:2c6e   | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 60   | singapore.com                         | 104.26.13.140             | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 75   | www.visa.com.hk                       | 104.18.21.69              | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 77   | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 97   | dnschecker.org                        | 2606:4700:20::681a:759    | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 114  | tasteatlas.com                        | 104.17.36.105             | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 198  | cmcc.877774.xyz                       | 104.16.149.244            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 5    | www.glassdoor.com                     | 104.17.64.70              | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 6    | www.glassdoor.com                     | 104.16.25.46              | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 48   | ip.sb                                 | 2606:4700:20::681a:d1f    | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 73   | whatismyipaddress.com                 | 2606:4700::6813:df4f      | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 91   | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1   | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 100  | uriah.ns.cloudflare.com               | 172.64.35.194             | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 190  | cmcc.877774.xyz                       | 104.16.148.8              | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 196  | cmcc.877774.xyz                       | 104.16.148.2              | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 8    | www.ipget.net                         | 104.21.15.212             | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 64   | 456.cloudflare.182682.xyz             | 104.26.9.160              | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 69   | 456.cloudflare.182682.xyz             | 2606:4700:20::681a:9a0    | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 106  | cf.877774.xyz                         | 104.18.41.190             | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 193  | cmcc.877774.xyz                       | 104.16.148.5              | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 197  | cmcc.877774.xyz                       | 104.16.148.1              | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 14   | comicabc.com                          | 2606:4700:3030::ac43:ae15 | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 57   | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 59   | singapore.com                         | 172.67.75.194             | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 65   | 456.cloudflare.182682.xyz             | 172.67.75.208             | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 74   | whatismyipaddress.com                 | 2606:4700::6813:de4f      | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 92   | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1   | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 10   | www.ipget.net                         | 2606:4700:3031::ac43:cf1a | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 39   | benedict.ns.cloudflare.com            | 162.159.44.205            | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 51   | 104.17.68.85                          | 104.17.68.85              | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 85   | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad   | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 194  | cmcc.877774.xyz                       | 104.16.148.4              | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 195  | cmcc.877774.xyz                       | 104.16.148.3              | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 63   | singapore.com                         | 2606:4700:20::681a:c8c    | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 191  | cmcc.877774.xyz                       | 104.16.148.7              | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 9    | www.ipget.net                         | 2606:4700:3036::6815:fd4  | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |
| 11   | comicabc.com                          | 188.114.96.3              | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 55   | ip.gs                                 | 2606:4700:3036::6815:eb0  | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |
| 71   | whatismyipaddress.com                 | 104.19.223.79             | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 83   | ashton.ns.cloudflare.com              | 172.64.35.173             | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 98   | dnschecker.org                        | 2606:4700:20::ac43:49d8   | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |
| 15   | palera.in                             | 172.67.157.122            | IPv4   | h2   | ✅ 成功 | 71       | cloudflare |
| 20   | trevor.ns.cloudflare.com              | 172.64.35.154             | IPv4   | h2   | ✅ 成功 | 71       | cloudflare |
| 61   | singapore.com                         | 2606:4700:20::681a:d8c    | IPv6   | h2   | ✅ 成功 | 71       | cloudflare |
| 102  | uriah.ns.cloudflare.com               | 108.162.195.194           | IPv4   | h2   | ✅ 成功 | 72       | cloudflare |
| 199  | cmcc.877774.xyz                       | 104.16.149.12             | IPv4   | h2   | ✅ 成功 | 73       | cloudflare |
| 94   | dnschecker.org                        | 172.67.73.216             | IPv4   | h2   | ✅ 成功 | 74       | cloudflare |
| 95   | dnschecker.org                        | 104.26.6.89               | IPv4   | h2   | ✅ 成功 | 74       | cloudflare |
| 58   | singapore.com                         | 104.26.12.140             | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 66   | 456.cloudflare.182682.xyz             | 104.26.8.160              | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 7    | www.ipget.net                         | 172.67.207.26             | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 19   | trevor.ns.cloudflare.com              | 162.159.44.154            | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 25   | iplocation.io                         | 104.26.10.222             | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 90   | julio.ns.cloudflare.com               | 172.64.35.209             | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 13   | comicabc.com                          | 2606:4700:3036::6815:400a | IPv6   | h2   | ✅ 成功 | 81       | cloudflare |
| 32   | wilson.ns.cloudflare.com              | 172.64.35.110             | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 33   | wilson.ns.cloudflare.com              | 108.162.195.110           | IPv4   | h2   | ✅ 成功 | 89       | cloudflare |
| 1    | 172.64.154.18                         | 172.64.154.18             | IPv4   | h2   | ✅ 成功 | 96       | cloudflare |
| 21   | trevor.ns.cloudflare.com              | 4444.cloudflare.182682.xyz           | IPv4   | h2   | ✅ 成功 | 97       | cloudflare |
| 36   | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e   | IPv6   | h2   | ✅ 成功 | 103      | cloudflare |
| 44   | benedict.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cd   | IPv6   | h2   | ✅ 成功 | 104      | cloudflare |
| 43   | benedict.ns.cloudflare.com            | 2606:4700:58::a29f:2ccd   | IPv6   | h2   | ✅ 成功 | 105      | cloudflare |
| 40   | benedict.ns.cloudflare.com            | 172.64.35.205             | IPv4   | h2   | ✅ 成功 | 106      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 96 条记录
- **正常 (100-200ms)**: 4 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 6 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 6 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
