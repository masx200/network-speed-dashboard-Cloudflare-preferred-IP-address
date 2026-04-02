# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 23:24:55
- **数据来源**: connectivity_results-20251212-232242.json
- **总测试数**: 452
- **失败测试数**: 7
- **成功测试数**: 445
- **失败率**: 1.55%
- **平均延迟**: 748.17ms
- **最小延迟**: 136ms
- **最大延迟**: 6788ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 23:24:55
- **IP地址**: 240e:38b:8b20:1200:6c97:1f15:bffd:8e32
- **国家/地区**: China (CN)
- **ASN**: AS4812
- **网络组织**: China Telecom Group
- **网络域名**: chinatelecom.com.cn
- **大洲**: Asia (AS)
- **地理坐标**: 31.2222, 121.4581
- **时区**: Asia/Shanghai
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 7 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (7 次测试)

| 序号 | 主机/域名                | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ------------------------ | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 72   | trevor.ns.cloudflare.com | 4444.cloudflare.182682.xyz | IPv4   | none | N/A    | 0        | N/A    | dial tcp 4444.cloudflare.182682.xyz:443: i/o timeout |
| 157  | cf.877774.xyz            | cf.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout   |
| 252  | cfip.xxxxxxxx.tk         | 104.20.255.53   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout   |
| 256  | cfip.xxxxxxxx.tk         | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 332  | 141.147.185.63           | 141.147.185.63  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 141.147.185.63:443: i/o timeout  |
| 435  | ct.877774.xyz            | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 447  | 3.0.50.69                | 3.0.50.69       | IPv4   | none | N/A    | 0        | N/A    | dial tcp 3.0.50.69:443: i/o timeout       |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 7 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 7 次超时，主要集中在IP段 108.162（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 7 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                   | 目标IP                    | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | --------------------------- | ------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 427  | 108.162.198.54              | 108.162.198.54            | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 287  | benedict.ns.cloudflare.com  | 162.159.44.205            | IPv4   | h3   | ✅ 成功 | 139      | cloudflare |
| 42   | ct.877774.xyz               | 172.64.229.173            | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 46   | ct.877774.xyz               | 172.64.229.217            | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 116  | cris.ns.cloudflare.com      | 162.159.44.202            | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 336  | uriah.ns.cloudflare.com     | 162.159.44.194            | IPv4   | h3   | ✅ 成功 | 149      | cloudflare |
| 43   | ct.877774.xyz               | 172.64.229.174            | IPv4   | h3   | ✅ 成功 | 155      | cloudflare |
| 305  | julio.ns.cloudflare.com     | 162.159.44.209            | IPv4   | h3   | ✅ 成功 | 157      | cloudflare |
| 27   | wilson.ns.cloudflare.com    | 162.159.44.110            | IPv4   | h3   | ✅ 成功 | 160      | cloudflare |
| 227  | moura.ns.cloudflare.com     | 162.159.44.217            | IPv4   | h3   | ✅ 成功 | 160      | cloudflare |
| 321  | ashton.ns.cloudflare.com    | 162.159.44.173            | IPv4   | h3   | ✅ 成功 | 160      | cloudflare |
| 45   | ct.877774.xyz               | 172.64.229.195            | IPv4   | h3   | ✅ 成功 | 161      | cloudflare |
| 47   | ct.877774.xyz               | 172.64.229.236            | IPv4   | h3   | ✅ 成功 | 164      | cloudflare |
| 48   | ct.877774.xyz               | 172.64.229.44             | IPv4   | h3   | ✅ 成功 | 164      | cloudflare |
| 161  | craig.ns.cloudflare.com     | 162.159.44.192            | IPv4   | h3   | ✅ 成功 | 167      | cloudflare |
| 9    | huxley.ns.cloudflare.com    | 162.159.44.188            | IPv4   | h3   | ✅ 成功 | 170      | cloudflare |
| 78   | sullivan.ns.cloudflare.com  | 162.159.44.161            | IPv4   | h3   | ✅ 成功 | 170      | cloudflare |
| 87   | pranab.ns.cloudflare.com    | 162.159.44.199            | IPv4   | h3   | ✅ 成功 | 173      | cloudflare |
| 220  | bowen.ns.cloudflare.com     | 162.159.44.83             | IPv4   | h3   | ✅ 成功 | 182      | cloudflare |
| 70   | trevor.ns.cloudflare.com    | 162.159.44.154            | IPv4   | h3   | ✅ 成功 | 190      | cloudflare |
| 235  | braden.ns.cloudflare.com    | 162.159.44.169            | IPv4   | h3   | ✅ 成功 | 194      | cloudflare |
| 337  | uriah.ns.cloudflare.com     | 172.64.35.194             | IPv4   | h3   | ✅ 成功 | 196      | cloudflare |
| 431  | abdullah.ns.cloudflare.com  | 162.159.44.203            | IPv4   | h3   | ✅ 成功 | 198      | cloudflare |
| 186  | dylan.ns.cloudflare.com     | 162.159.44.187            | IPv4   | h3   | ✅ 成功 | 199      | cloudflare |
| 274  | rustam.ns.cloudflare.com    | 162.159.44.148            | IPv4   | h3   | ✅ 成功 | 200      | cloudflare |
| 393  | otto.ns.cloudflare.com      | 162.159.44.135            | IPv4   | h3   | ✅ 成功 | 202      | cloudflare |
| 407  | lewis.ns.cloudflare.com     | 162.159.44.159            | IPv4   | h3   | ✅ 成功 | 204      | cloudflare |
| 411  | lewis.ns.cloudflare.com     | 2803:f800:50::6ca2:c39f   | IPv6   | h3   | ✅ 成功 | 209      | cloudflare |
| 49   | ct.877774.xyz               | 172.64.229.161            | IPv4   | h3   | ✅ 成功 | 210      | cloudflare |
| 44   | ct.877774.xyz               | 172.64.229.185            | IPv4   | h3   | ✅ 成功 | 217      | cloudflare |
| 173  | decker.ns.cloudflare.com    | 162.159.44.155            | IPv4   | h3   | ✅ 成功 | 219      | cloudflare |
| 26   | wilson.ns.cloudflare.com    | 108.162.195.110           | IPv4   | h3   | ✅ 成功 | 223      | cloudflare |
| 176  | decker.ns.cloudflare.com    | 2a06:98c1:50::ac40:239b   | IPv6   | h3   | ✅ 成功 | 223      | cloudflare |
| 400  | damien.ns.cloudflare.com    | 108.162.195.168           | IPv4   | h3   | ✅ 成功 | 228      | cloudflare |
| 4    | 104.18.37.13                | 104.18.37.13              | IPv4   | h3   | ✅ 成功 | 233      | cloudflare |
| 171  | kyree.ns.cloudflare.com     | 2606:4700:58::a29f:2ccf   | IPv6   | h3   | ✅ 成功 | 234      | cloudflare |
| 402  | damien.ns.cloudflare.com    | 162.159.44.168            | IPv4   | h3   | ✅ 成功 | 234      | cloudflare |
| 170  | kyree.ns.cloudflare.com     | 2a06:98c1:50::ac40:23cf   | IPv6   | h3   | ✅ 成功 | 236      | cloudflare |
| 241  | cf.zhetengsha.eu.org        | 2a06:98c1:310d::6812:2bae | IPv6   | h3   | ✅ 成功 | 238      | cloudflare |
| 166  | kyree.ns.cloudflare.com     | 162.159.44.207            | IPv4   | h3   | ✅ 成功 | 240      | cloudflare |
| 239  | cf.zhetengsha.eu.org        | 104.18.42.98              | IPv4   | h3   | ✅ 成功 | 240      | cloudflare |
| 120  | cris.ns.cloudflare.com      | 2a06:98c1:50::ac40:23ca   | IPv6   | h3   | ✅ 成功 | 241      | cloudflare |
| 188  | dylan.ns.cloudflare.com     | 2606:4700:58::a29f:2cbb   | IPv6   | h3   | ✅ 成功 | 241      | cloudflare |
| 275  | rustam.ns.cloudflare.com    | 172.64.35.148             | IPv4   | h3   | ✅ 成功 | 241      | cloudflare |
| 346  | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6   | h3   | ✅ 成功 | 241      | cloudflare |
| 380  | 104.18.37.40                | 104.18.37.40              | IPv4   | h3   | ✅ 成功 | 241      | cloudflare |
| 429  | abdullah.ns.cloudflare.com  | 172.64.35.203             | IPv4   | h3   | ✅ 成功 | 242      | cloudflare |
| 338  | uriah.ns.cloudflare.com     | 2803:f800:50::6ca2:c3c2   | IPv6   | h3   | ✅ 成功 | 243      | cloudflare |
| 7    | huxley.ns.cloudflare.com    | 108.162.195.188           | IPv4   | h3   | ✅ 成功 | 245      | cloudflare |
| 226  | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6   | h3   | ✅ 成功 | 245      | cloudflare |
| 356  | cf.877774.xyz               | 2606:4700:4406::ac40:9242 | IPv6   | h3   | ✅ 成功 | 245      | cloudflare |
| 81   | sullivan.ns.cloudflare.com  | 2803:f800:50::6ca2:c3a1   | IPv6   | h3   | ✅ 成功 | 246      | cloudflare |
| 189  | dylan.ns.cloudflare.com     | 2803:f800:50::6ca2:c3bb   | IPv6   | h3   | ✅ 成功 | 246      | cloudflare |
| 185  | dylan.ns.cloudflare.com     | 108.162.195.187           | IPv4   | h3   | ✅ 成功 | 247      | cloudflare |
| 225  | 172.64.151.55               | 172.64.151.55             | IPv4   | h3   | ✅ 成功 | 249      | cloudflare |
| 242  | cf.zhetengsha.eu.org        | 2606:4700:4407::ac40:9052 | IPv6   | h3   | ✅ 成功 | 249      | cloudflare |
| 160  | craig.ns.cloudflare.com     | 108.162.195.192           | IPv4   | h3   | ✅ 成功 | 250      | cloudflare |
| 162  | craig.ns.cloudflare.com     | 172.64.35.192             | IPv4   | h3   | ✅ 成功 | 253      | cloudflare |
| 277  | rustam.ns.cloudflare.com    | 2803:f800:50::6ca2:c394   | IPv6   | h3   | ✅ 成功 | 253      | cloudflare |
| 169  | kyree.ns.cloudflare.com     | 2803:f800:50::6ca2:c3cf   | IPv6   | h3   | ✅ 成功 | 255      | cloudflare |
| 320  | ashton.ns.cloudflare.com    | 108.162.195.173           | IPv4   | h3   | ✅ 成功 | 255      | cloudflare |
| 329  | 104.18.42.26                | 104.18.42.26              | IPv4   | h3   | ✅ 成功 | 255      | cloudflare |
| 28   | wilson.ns.cloudflare.com    | 2606:4700:58::a29f:2c6e   | IPv6   | h3   | ✅ 成功 | 256      | cloudflare |
| 408  | lewis.ns.cloudflare.com     | 172.64.35.159             | IPv4   | h3   | ✅ 成功 | 256      | cloudflare |
| 355  | cf.877774.xyz               | 2a06:98c1:3102::6812:29be | IPv6   | h3   | ✅ 成功 | 257      | cloudflare |
| 381  | 172.64.35.24                | 172.64.35.24              | IPv4   | h3   | ✅ 成功 | 257      | cloudflare |
| 92   | pranab.ns.cloudflare.com    | 2606:4700:58::a29f:2cc7   | IPv6   | h3   | ✅ 成功 | 258      | cloudflare |
| 114  | na.877774.xyz               | 104.18.38.235             | IPv4   | h3   | ✅ 成功 | 258      | cloudflare |
| 177  | decker.ns.cloudflare.com    | 2606:4700:58::a29f:2c9b   | IPv6   | h3   | ✅ 成功 | 259      | cloudflare |
| 233  | braden.ns.cloudflare.com    | 108.162.195.169           | IPv4   | h3   | ✅ 成功 | 259      | cloudflare |
| 403  | damien.ns.cloudflare.com    | 2803:f800:50::6ca2:c3a8   | IPv6   | h3   | ✅ 成功 | 259      | cloudflare |
| 290  | benedict.ns.cloudflare.com  | 2606:4700:58::a29f:2ccd   | IPv6   | h3   | ✅ 成功 | 260      | cloudflare |
| 29   | wilson.ns.cloudflare.com    | 2803:f800:50::6ca2:c36e   | IPv6   | h3   | ✅ 成功 | 261      | cloudflare |
| 418  | www.wto.org                 | 2a06:98c1:3102::6812:29be | IPv6   | h3   | ✅ 成功 | 261      | cloudflare |
| 339  | uriah.ns.cloudflare.com     | 2a06:98c1:50::ac40:23c2   | IPv6   | h3   | ✅ 成功 | 262      | cloudflare |
| 228  | moura.ns.cloudflare.com     | 172.64.35.217             | IPv4   | h3   | ✅ 成功 | 263      | cloudflare |
| 187  | dylan.ns.cloudflare.com     | 172.64.35.187             | IPv4   | h3   | ✅ 成功 | 264      | cloudflare |
| 416  | www.wto.org                 | 104.18.41.190             | IPv4   | h3   | ✅ 成功 | 264      | cloudflare |
| 307  | julio.ns.cloudflare.com     | 2606:4700:58::a29f:2cd1   | IPv6   | h3   | ✅ 成功 | 265      | cloudflare |
| 415  | www.wto.org                 | 172.64.146.66             | IPv4   | h3   | ✅ 成功 | 265      | cloudflare |
| 12   | huxley.ns.cloudflare.com    | 2a06:98c1:50::ac40:23bc   | IPv6   | h3   | ✅ 成功 | 266      | cloudflare |
| 167  | kyree.ns.cloudflare.com     | 172.64.35.207             | IPv4   | h3   | ✅ 成功 | 266      | cloudflare |
| 229  | moura.ns.cloudflare.com     | 108.162.195.217           | IPv4   | h3   | ✅ 成功 | 266      | cloudflare |
| 276  | rustam.ns.cloudflare.com    | 2606:4700:58::a29f:2c94   | IPv6   | h3   | ✅ 成功 | 266      | cloudflare |
| 334  | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6   | h3   | ✅ 成功 | 266      | cloudflare |
| 430  | abdullah.ns.cloudflare.com  | 108.162.195.203           | IPv4   | h3   | ✅ 成功 | 266      | cloudflare |
| 30   | wilson.ns.cloudflare.com    | 2a06:98c1:50::ac40:236e   | IPv6   | h3   | ✅ 成功 | 267      | cloudflare |
| 174  | decker.ns.cloudflare.com    | 172.64.35.155             | IPv4   | h3   | ✅ 成功 | 267      | cloudflare |
| 244  | saas.sin.fan                | 162.159.36.20             | IPv4   | h3   | ✅ 成功 | 267      | cloudflare |
| 8    | huxley.ns.cloudflare.com    | 172.64.35.188             | IPv4   | h3   | ✅ 成功 | 268      | cloudflare |
| 308  | julio.ns.cloudflare.com     | 2a06:98c1:50::ac40:23d1   | IPv6   | h3   | ✅ 成功 | 269      | cloudflare |
| 353  | cf.877774.xyz               | 172.64.146.66             | IPv4   | h3   | ✅ 成功 | 269      | cloudflare |
| 397  | otto.ns.cloudflare.com      | 2803:f800:50::6ca2:c387   | IPv6   | h3   | ✅ 成功 | 269      | cloudflare |
| 190  | dylan.ns.cloudflare.com     | 2a06:98c1:50::ac40:23bb   | IPv6   | h3   | ✅ 成功 | 271      | cloudflare |
| 410  | lewis.ns.cloudflare.com     | 2606:4700:58::a29f:2c9f   | IPv6   | h3   | ✅ 成功 | 271      | cloudflare |
| 118  | cris.ns.cloudflare.com      | 2803:f800:50::6ca2:c3ca   | IPv6   | h3   | ✅ 成功 | 272      | cloudflare |
| 172  | decker.ns.cloudflare.com    | 108.162.195.155           | IPv4   | h3   | ✅ 成功 | 272      | cloudflare |
| 223  | bowen.ns.cloudflare.com     | 2606:4700:58::a29f:2c53   | IPv6   | h3   | ✅ 成功 | 272      | cloudflare |
| 237  | braden.ns.cloudflare.com    | 2803:f800:50::6ca2:c3a9   | IPv6   | h3   | ✅ 成功 | 272      | cloudflare |
| 392  | otto.ns.cloudflare.com      | 172.64.35.135             | IPv4   | h3   | ✅ 成功 | 272      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 24 条记录
- **慢 (200-500ms)**: 76 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 7 次
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
