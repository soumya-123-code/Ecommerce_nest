import { useState } from 'react';
import {
  Facebook,
  Twitter,
  Instagram,
  Youtube,
  Send,
  Package,
  Truck,
  Gift,
  RotateCcw,
  ShieldCheck
} from 'lucide-react';

export function Footer() {
  const [email, setEmail] = useState('');

  const handleNewsletterSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    console.log('Newsletter subscription:', email);
    // TODO: Implement newsletter subscription mutation
    setEmail('');
  };

  return (
    <footer className="main">
      {/* Newsletter Section */}
      <section className="newsletter mb-15 wow animate__animated animate__fadeIn bg-brand-light py-20">
        <div className="container mx-auto px-4">
          <div className="newsletter-inner relative bg-white rounded-lg shadow-lg overflow-hidden">
            <div className="flex flex-col lg:flex-row items-center justify-between p-12">
              <div className="newsletter-content flex-1 mb-8 lg:mb-0">
                <h2 className="text-4xl font-bold mb-4">
                  Stay home & get your daily <br />
                  needs from our shop
                </h2>
                <p className="text-gray-600 mb-8">
                  Start Your Daily Shopping with <span className="text-brand font-semibold">eCommerce</span>
                </p>
                <form onSubmit={handleNewsletterSubmit} className="form-subcriber flex max-w-md">
                  <input
                    type="email"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    placeholder="Your email address"
                    className="flex-1 px-6 py-4 border border-gray-300 rounded-l-full focus:outline-none focus:border-brand"
                    required
                  />
                  <button
                    type="submit"
                    className="bg-brand hover:bg-brand-dark text-white px-8 py-4 rounded-r-full font-semibold transition-colors flex items-center"
                  >
                    <Send className="w-5 h-5 mr-2" />
                    Subscribe
                  </button>
                </form>
              </div>
              <div className="newsletter-image hidden lg:block">
                <img
                  src="/assets/imgs/banner/banner-9.png"
                  alt="newsletter"
                  className="w-64 h-auto"
                  onError={(e) => {
                    (e.target as HTMLImageElement).style.display = 'none';
                  }}
                />
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="featured section-padding bg-gray-50 py-12">
        <div className="container mx-auto px-4">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-6">
            {/* Feature 1 */}
            <div className="banner-left-icon flex items-center p-4 bg-white rounded-lg">
              <div className="banner-icon mr-4">
                <Package className="w-12 h-12 text-brand" />
              </div>
              <div className="banner-text">
                <h3 className="font-semibold text-lg mb-1">Best prices & offers</h3>
                <p className="text-sm text-gray-600">Orders $50 or more</p>
              </div>
            </div>

            {/* Feature 2 */}
            <div className="banner-left-icon flex items-center p-4 bg-white rounded-lg">
              <div className="banner-icon mr-4">
                <Truck className="w-12 h-12 text-brand" />
              </div>
              <div className="banner-text">
                <h3 className="font-semibold text-lg mb-1">Free delivery</h3>
                <p className="text-sm text-gray-600">24/7 amazing services</p>
              </div>
            </div>

            {/* Feature 3 */}
            <div className="banner-left-icon flex items-center p-4 bg-white rounded-lg">
              <div className="banner-icon mr-4">
                <Gift className="w-12 h-12 text-brand" />
              </div>
              <div className="banner-text">
                <h3 className="font-semibold text-lg mb-1">Great daily deal</h3>
                <p className="text-sm text-gray-600">When you sign up</p>
              </div>
            </div>

            {/* Feature 4 */}
            <div className="banner-left-icon flex items-center p-4 bg-white rounded-lg">
              <div className="banner-icon mr-4">
                <ShieldCheck className="w-12 h-12 text-brand" />
              </div>
              <div className="banner-text">
                <h3 className="font-semibold text-lg mb-1">Wide assortment</h3>
                <p className="text-sm text-gray-600">Mega Discounts</p>
              </div>
            </div>

            {/* Feature 5 */}
            <div className="banner-left-icon flex items-center p-4 bg-white rounded-lg">
              <div className="banner-icon mr-4">
                <RotateCcw className="w-12 h-12 text-brand" />
              </div>
              <div className="banner-text">
                <h3 className="font-semibold text-lg mb-1">Easy returns</h3>
                <p className="text-sm text-gray-600">Within 30 days</p>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Footer Middle Section */}
      <section className="section-padding footer-mid bg-white py-16">
        <div className="container mx-auto px-4">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-8">
            {/* Company Info */}
            <div className="col-span-1 lg:col-span-2">
              <div className="widget-about font-md mb-8">
                <div className="logo mb-6">
                  <a href="/">
                    <img
                      src="/assets/imgs/theme/logo.svg"
                      alt="eCommerce"
                      className="h-10"
                      onError={(e) => {
                        (e.target as HTMLImageElement).src = 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" width="120" height="40"><text x="10" y="25" font-size="20" fill="%233BB77E">eCommerce</text></svg>';
                      }}
                    />
                  </a>
                </div>
                <p className="text-gray-600 mb-4">
                  Awesome eCommerce store built with Django, GraphQL, and React.
                  Your one-stop shop for all your needs.
                </p>
                <p className="text-gray-700 mb-2">
                  <strong>Address:</strong> 123 Market Street, City, State 12345
                </p>
                <p className="text-gray-700 mb-2">
                  <strong>Phone:</strong> +1 (800) 123-4567
                </p>
                <p className="text-gray-700 mb-4">
                  <strong>Email:</strong> support@ecommerce.com
                </p>
                <div className="flex space-x-4">
                  <a href="#" className="text-gray-400 hover:text-brand">
                    <Facebook className="w-6 h-6" />
                  </a>
                  <a href="#" className="text-gray-400 hover:text-brand">
                    <Twitter className="w-6 h-6" />
                  </a>
                  <a href="#" className="text-gray-400 hover:text-brand">
                    <Instagram className="w-6 h-6" />
                  </a>
                  <a href="#" className="text-gray-400 hover:text-brand">
                    <Youtube className="w-6 h-6" />
                  </a>
                </div>
              </div>
            </div>

            {/* Company Links */}
            <div>
              <h5 className="widget-title font-semibold text-lg mb-4">Company</h5>
              <ul className="footer-list space-y-2">
                <li><a href="#" className="text-gray-600 hover:text-brand">About Us</a></li>
                <li><a href="#" className="text-gray-600 hover:text-brand">Delivery Information</a></li>
                <li><a href="#" className="text-gray-600 hover:text-brand">Privacy Policy</a></li>
                <li><a href="#" className="text-gray-600 hover:text-brand">Terms & Conditions</a></li>
                <li><a href="#" className="text-gray-600 hover:text-brand">Contact Us</a></li>
              </ul>
            </div>

            {/* Account Links */}
            <div>
              <h5 className="widget-title font-semibold text-lg mb-4">Account</h5>
              <ul className="footer-list space-y-2">
                <li><a href="#" className="text-gray-600 hover:text-brand">Sign In</a></li>
                <li><a href="#" className="text-gray-600 hover:text-brand">View Cart</a></li>
                <li><a href="#" className="text-gray-600 hover:text-brand">My Wishlist</a></li>
                <li><a href="#" className="text-gray-600 hover:text-brand">Track My Order</a></li>
                <li><a href="#" className="text-gray-600 hover:text-brand">Help</a></li>
              </ul>
            </div>

            {/* Corporate Links */}
            <div>
              <h5 className="widget-title font-semibold text-lg mb-4">Corporate</h5>
              <ul className="footer-list space-y-2">
                <li><a href="#" className="text-gray-600 hover:text-brand">Become a Vendor</a></li>
                <li><a href="#" className="text-gray-600 hover:text-brand">Affiliate Program</a></li>
                <li><a href="#" className="text-gray-600 hover:text-brand">Farm Business</a></li>
                <li><a href="#" className="text-gray-600 hover:text-brand">Farm Careers</a></li>
                <li><a href="#" className="text-gray-600 hover:text-brand">Our Suppliers</a></li>
              </ul>
            </div>
          </div>
        </div>
      </section>

      {/* Footer Bottom */}
      <div className="footer-bottom bg-gray-100 py-6 border-t">
        <div className="container mx-auto px-4">
          <div className="flex flex-col md:flex-row justify-between items-center">
            <p className="text-gray-600 text-sm mb-4 md:mb-0">
              © 2024 <strong className="text-brand">eCommerce</strong>. All rights reserved.
            </p>
            <div className="flex items-center space-x-4">
              <span className="text-gray-600 text-sm">We accept:</span>
              <div className="flex space-x-2">
                <img src="/assets/imgs/theme/payment-method.png" alt="Payment Methods" className="h-8"
                  onError={(e) => {
                    (e.target as HTMLImageElement).style.display = 'none';
                  }}
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    </footer>
  );
}
