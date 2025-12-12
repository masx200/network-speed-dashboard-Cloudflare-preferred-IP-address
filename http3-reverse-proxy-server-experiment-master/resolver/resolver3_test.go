package resolver

import (
	"log"
	"testing"

	dns_experiment "github.com/masx200/http3-reverse-proxy-server-experiment/dns"
	"github.com/masx200/http3-reverse-proxy-server-experiment/generic"
	"github.com/miekg/dns"
)

func TestResolver7(t *testing.T) {
	x := "www.google.com"
	results, err := dns_experiment.DnsResolverMultipleServers(x, GetQueryCallbacks7(), func(dro *dns_experiment.DnsResolverOptions) {
		dro.DnsCache = DnsCache
	})

	if err != nil {
		log.Printf("Error: %v\n", err)
		t.Error(err)
		return
	}

	for _, result := range results {
		log.Println(x, result)
	}
}
func TestResolver27(t *testing.T) {
	x := "www.render.com"
	results, err := dns_experiment.DnsResolverMultipleServers(x, GetQueryCallbacks7(), func(dro *dns_experiment.DnsResolverOptions) {
		dro.DnsCache = DnsCache
	})

	if err != nil {
		log.Printf("Error: %v\n", err)
		t.Error(err)
		return
	}

	for _, result := range results {
		log.Println(x, result)
	}
}
func TestResolver73(t *testing.T) {
	x := "www.so.com"
	results, err := dns_experiment.DnsResolverMultipleServers(x, GetQueryCallbacks7(), func(dro *dns_experiment.DnsResolverOptions) {
		dro.DnsCache = DnsCache
	})

	if err != nil {
		log.Printf("Error: %v\n", err)
		t.Error(err)
		return
	}

	for _, result := range results {
		log.Println(x, result)
	}
}
func GetQueryCallbacks7() generic.MapInterface[string, func(m *dns.Msg) (r *dns.Msg, err error)] {
	return generic.MapImplementFromMap(map[string]func(m *dns.Msg) (r *dns.Msg, err error){
		"https://deno-dns-over-https-server.g18uibxgnb.de5.net/": func(m *dns.Msg) (r *dns.Msg, err error) {
			return DohClient(m, "https://deno-dns-over-https-server.g18uibxgnb.de5.net/")
		}, "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": func(m *dns.Msg) (r *dns.Msg, err error) {
			return DohClient(m, "https://deno-dns-over-https-server.g18uibxgnb.de5.net/")
		}, "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": func(m *dns.Msg) (r *dns.Msg, err error) {
			return DoHTTP3Client(m, "https://deno-dns-over-https-server.g18uibxgnb.de5.net/")
		}, "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": func(m *dns.Msg) (r *dns.Msg, err error) {
			return DoHTTP3Client(m, "https://deno-dns-over-https-server.g18uibxgnb.de5.net/")
		}})
}
func TestResolver47(t *testing.T) {
	x := "local-aria2-webui.masx200.ddns-ip.net"
	results, err := dns_experiment.DnsResolverMultipleServers(x, GetQueryCallbacks7(), func(dro *dns_experiment.DnsResolverOptions) {
		dro.DnsCache = DnsCache
	})

	if err != nil {
		log.Printf("Error: %v\n", err)
		t.Error(err)
		return
	}

	for _, result := range results {
		log.Println(x, result)
	}
}
func TestResolverMultipleServers7(t *testing.T) {
	x := "www.360.cn"
	results, err := dns_experiment.DnsResolverMultipleServers(x, GetQueryCallbacks7(), func(dro *dns_experiment.DnsResolverOptions) {
		dro.DnsCache = DnsCache
	})

	if err != nil {
		log.Printf("Error: %v\n", err)
		t.Error(err)
		return
	}

	for _, result := range results {
		log.Println(x, result)
	}
}
